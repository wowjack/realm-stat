use std::collections::VecDeque;
use byteorder::{BigEndian, ByteOrder};
use crate::rc4::Rc4;
use super::{rotmg_packet::RotmgPacket, byte_buffer::ByteBuffer, rotmg_packet_stitcher::StitchedPacket};


const IKEY: [u8; 13] = [0xc9, 0x1d, 0x9e, 0xec, 0x42, 0x01, 0x60, 0x73, 0x0d, 0x82, 0x56, 0x04, 0xe0];
const OKEY: [u8; 13] = [0x5a, 0x4d, 0x20, 0x16, 0xbc, 0x16, 0xdc, 0x64, 0x88, 0x31, 0x94, 0xff, 0xd9];


/**
 * Takes in application packets that have been stitched together by the packet factory
 * Decrypts the packets and waits to output them
 * When a tick packet arrives, cipher alignment is checked and stored packets are sent out if the cipher is correct
 * 
 * Maintains cipher and tick alignment
 */
pub struct RotmgPacketConstructor {
    iqueue: VecDeque<StitchedPacket>,
    oqueue: VecDeque<RotmgPacket>,

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
        if packet.type_num == 10 { //NewTick packet type number
            self.check_queue();
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
    fn check_queue(&mut self) {
        let tick = self.iqueue.back().expect("Unexpected error in packet constructor.").clone();
        let bytes_in_queue_except_tick = self.iqueue.iter().take(self.iqueue.len()-1).map(|i| i.data.rem_len()).sum();
        let mut new_cipher = self.cipher.clone();
        new_cipher.skip(bytes_in_queue_except_tick);

        self.current_tick = match self.current_tick {None=>None, Some(n)=>Some(n+1)};
        let new_tick = BigEndian::read_u32(&self.cipher.apply_keystream_static(0, &tick.data.read_n_bytes_static(4).unwrap().to_vec()));
        
        if let Some((old_tick_data, cipher)) = &self.prev_tick_info {
            if let Some(current_tick) = self.current_tick {
                //current tick and prev tick info is defined, expect cipher to be correct
                if current_tick != new_tick {
                    //need to realign
                    log::debug!("Tick packet alignment failure. Expected {current_tick} got {new_tick}");
                    self.realign(tick.data.clone(), bytes_in_queue_except_tick + old_tick_data.len() - 9);
                    self.prev_tick_info = Some((tick.data.clone(), new_cipher));
                    return
                } else {
                    //Ticks and cipher are aligned, decrypt and drain the queue
                    self.prev_tick_info = Some((tick.data.clone(), new_cipher));
                    self.drain_queue();
                }
            } else {
                //current tick is undefined so this is the second tick packet
                //need to realign
                log::debug!("Got second tick packet");
                self.realign(tick.data.clone(), bytes_in_queue_except_tick + old_tick_data.len() - 9);
                self.prev_tick_info = Some((tick.data.clone(), new_cipher));
            }
        } else {
            if let Some(_) = self.current_tick {
                //current tick is defined but prev tick isnt, how did you get here?
                panic!("How did you get here?");
            } else {
                log::debug!("Got first tick packet");
                //this is the first tick packet seen
                if new_tick == 0 {
                    //Decryption is probably correct if the first tick packet decrypts to a tick id of 0
                    self.current_tick = Some(0);
                }
                self.iqueue.clear();
                self.prev_tick_info = Some((tick.data.clone(), new_cipher.clone()))
            }
        }
    }

    fn drain_queue(&mut self) {
        log::debug!("Draining queue");
        for p in self.iqueue.drain(..) {
            let data = ByteBuffer::new(self.cipher.apply_keystream(5, &p.data.to_vec()));
            if let Ok(rp) = RotmgPacket::try_from(data) {
                log::debug!("Produced packet {:?}", rp);
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
    fn realign(&mut self, tick_data: ByteBuffer, bytes_between: usize) {
        
        if let Some((old_bytes, cipher)) = &mut self.prev_tick_info {

            log::debug!("Realigning cipher");
            log::debug!("Bytese between: {bytes_between}");
            log::debug!("old tick: {:?}", old_bytes);
            log::debug!("new tick: {:?}", tick_data);
            for p in self.iqueue.iter() {
                log::debug!("iqueue: {:?}", p);
            }
            let c1: Vec<u8> = old_bytes.read_n_bytes_static(4).unwrap().to_vec();
            let c2: Vec<u8> = tick_data.read_n_bytes_static(4).unwrap().to_vec();

            //First try to realign the cipher by trying a bunch of different offsets for the second tick until key^second_tick = self.current_tick 
            if let Some(t) = self.current_tick {
                let mut new_cipher = cipher.clone();
                let result = new_cipher.align_to_real_tick(t, &c2);
                if result == true {
                    new_cipher.skip(tick_data.len()-5); //align_to_real_tick hands the cipher back before it has decrypted the tick packet
                    self.cipher = new_cipher;
                    self.iqueue.clear();
                    return
                }
            }

            if c1 == c2 {
                log::debug!("old and new cipher are the same, resetting");
                self.reset();
                return;
            }
            
            self.cipher.reset();
            self.cipher.align_to(&c1, &c2, bytes_between + old_bytes.len() - 9);
            self.current_tick = Some(self.cipher.align_to(&c1, &c2, bytes_between + old_bytes.len() - 9));
            self.cipher.skip(tick_data.len()-5);
        }
    }

    pub fn reset(&mut self) {
        self.cipher.reset();
        self.iqueue.clear();
        self.current_tick = None;
        self.prev_tick_info = None;
    }
}