use std::collections::VecDeque;
use byteorder::{BigEndian, ByteOrder};
use crate::rc4::Rc4;
use super::{rotmg_packet::RotmgPacket, byte_buffer::ByteBuffer, rotmg_packet_stitcher::StitchedPacket};


const IKEY: [u8; 13] = [0xc9, 0x1d, 0x9e, 0xec, 0x42, 0x01, 0x60, 0x73, 0x0d, 0x82, 0x56, 0x04, 0xe0];
const _OKEY: [u8; 13] = [0x5a, 0x4d, 0x20, 0x16, 0xbc, 0x16, 0xdc, 0x64, 0x88, 0x31, 0x94, 0xff, 0xd9];


/**
 * Takes in application packets that have been stitched together by the packet factory
 * Decrypts the packets and waits to output them
 * When a tick packet arrives, cipher alignment is checked and stored packets are sent out if the cipher is correct
 * 
 * Maintains cipher and tick alignment
 */
pub struct RotmgPacketConstructor {
    iqueue: VecDeque<StitchedPacket>,
    pub oqueue: VecDeque<RotmgPacket>,

    cipher: Rc4,
    current_tick: Option<u32>,
    //Encrypted tick packet, bytes received since the tick packet was received, cipher that would have been used to decrypt
    //This rc4 cipher is guaranteed to be behind the correct one 
    prev_tick_info: Option<(ByteBuffer, Rc4)>,
}
impl RotmgPacketConstructor {
    pub fn new() -> Self {
        Self {
            iqueue: VecDeque::new(),
            oqueue: VecDeque::new(),
            cipher: Rc4::new(Vec::from(IKEY)),
            current_tick: None, prev_tick_info: None
        }
    }

    /**
     * Add stitched packet to the input queue
     * Wait until a tick packet has been received and validated before flushing the queue
     * If the tick packet couldn't be validated, go through realignment steps
     */
    pub fn insert_packet(&mut self, packet: StitchedPacket) {
        self.iqueue.push_back(packet.clone());
        //log::debug!("Received packet: {:?}", packet);
        if packet.type_num == 10 { //NewTick packet type number
            self.process_tick(packet);
        } else if packet.type_num == 45 { //Reconnect packet type number
            self.reset();
        }
    }
    pub fn get_packet(&mut self) -> Option<RotmgPacket> {
        self.oqueue.pop_front()
    }

    /**
     * Runs every time a tick packet is received to check the validity of the tick packet.
     * If the tick packet is valid, each packet in the queue will be decrypted and flushed to the output queue
     * If the tick is invalid, try to realign the cipher
     */
    fn process_tick(&mut self, tick: StitchedPacket) {
        //Sometimes duplicate packets arrive
        //If we get a duplicate tick, clear the entire queue
        if let Some((old_tick, _)) = &self.prev_tick_info {
            if old_tick == &tick.data {
                log::debug!("Duplicate tick!");
                self.iqueue.clear();
                return;
            }
        }


        let bytes_in_queue_except_tick = self.iqueue.iter().take(self.iqueue.len()-1).map(|i| i.data.rem_len()).sum();
        let mut new_cipher = self.cipher.clone();
        new_cipher.skip(bytes_in_queue_except_tick);

        self.current_tick = match self.current_tick {None=>None, Some(n)=>Some(n+1)};
        let new_tick = BigEndian::read_u32(&new_cipher.apply_keystream_static(0, &tick.data.read_n_bytes_static(4).unwrap().to_vec()));
        
        match self.current_tick {
            Some(t) => {
                if t == new_tick {
                    //alignment is all good
                    self.drain_queue();
                    self.prev_tick_info = Some((tick.data.clone(), new_cipher));
                } else {
                    //need to realign using real tick
                    log::debug!("Tick counter alignment failure. Got {new_tick} expected {t}");
                    self.try_realign(tick.data, bytes_in_queue_except_tick);
                }
            },
            None => {
                if new_tick == 0 {
                    //alignment is probably okay
                    self.current_tick = Some(0);
                    self.prev_tick_info = Some((tick.data.clone(), new_cipher));
                } else {
                    //try to brute force realign
                    self.try_realign(tick.data, bytes_in_queue_except_tick);
                }
            }
        }
    }

    fn drain_queue(&mut self) {
        //log::debug!("Draining queue");
        for p in self.iqueue.drain(..) {
            let data = ByteBuffer::new(self.cipher.apply_keystream(5, &p.data.to_vec()));
            if let Ok(rp) = RotmgPacket::try_from(data) {
                //log::debug!("Produced packet {:?}", rp);
                self.oqueue.push_back(rp);
            } else {
                log::debug!("Error constructing packet");
            }
        }
    }

    /**
     * Attempts to realign the rc4 cipher and current tick counter using the stored previous tick packet.
     * 
     * If self.prev_tick_packet == None, tick packet is the first tick packet so it is incorrectly decrypted unless it is 0
     * 
     * Perhaps attempt to realign in a separate thread to allow reset packets to be processed to reset the cipher. Queueing all packets while realigning may cause memory issues.
     */
    fn try_realign(&mut self, tick_data: ByteBuffer, bytes_between: usize) {
        //log::debug!("Bytes between ticks: {}", bytes_between + tick_data.rem_len()-4);
        if let Some((old_bytes, old_cipher)) = self.prev_tick_info.clone() {
            if let Some(expected_tick) = self.current_tick {
                log::debug!("Attempting to align to real tick");
                let mut tmp_cipher = old_cipher.clone();
                if old_cipher.apply_keystream_static(0, &old_bytes.read_n_bytes_static(4).unwrap().to_vec()) != (expected_tick - 1).to_be_bytes().to_vec() {
                    panic!("Old cipher does not correctly decrypt old tick {}", expected_tick-1);
                }
                if tmp_cipher.align_to_real_tick(expected_tick, &tick_data.read_n_bytes_static(4).unwrap()) == true {
                    log::debug!("Success!");
                    self.prev_tick_info = Some((tick_data.clone(), tmp_cipher.clone()));
                    tmp_cipher.skip(tick_data.rem_len());

                    //The iqueue must be cleared after a real tick align
                    //Something went wrong with the amount of data in between the two tick packets, so you cannot expect to decrypt those packets without problems
                    self.iqueue.clear();
                    return
                }
            }

            log::debug!("Brute force realigning");
            //self.cipher.reset();
            let new_tick = self.cipher.align_to(&old_bytes.read_n_bytes_static(4).unwrap(), &tick_data.read_n_bytes_static(4).unwrap(), bytes_between + old_bytes.rem_len()-4);
            if new_tick == u32::MAX {
                //alignment failed
                self.reset();
                self.prev_tick_info = Some((tick_data, self.cipher.clone()));
            } else {
                log::debug!("Success!");
                self.current_tick = Some(new_tick);
                self.cipher.skip(old_bytes.rem_len());
                let mut tmp_cipher = self.cipher.clone();
                tmp_cipher.skip(bytes_between);
                self.prev_tick_info = Some((tick_data.clone(), tmp_cipher));
                self.drain_queue();
            }

        } else {
            log::debug!("Attempted to realign without previous tick info");
            let mut tmp_cipher = self.cipher.clone();
            tmp_cipher.reset();
            self.prev_tick_info = Some((tick_data, tmp_cipher));
            self.iqueue.clear()
        }
    }

    pub fn reset(&mut self) {
        self.cipher.reset();
        self.iqueue.clear();
        self.current_tick = None;
        self.prev_tick_info = None;
    }
}