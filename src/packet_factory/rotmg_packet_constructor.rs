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

    pub cipher: Rc4,
    current_tick: Option<u32>,

    //For detecting duplicate tick packets
    old_tick_data: Option<ByteBuffer>,
}
impl RotmgPacketConstructor {
    pub fn new() -> Self {
        Self {
            iqueue: VecDeque::new(),
            oqueue: VecDeque::new(),
            cipher: Rc4::new(Vec::from(IKEY)),
            current_tick: None,
            old_tick_data: None,
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
        if let Some(old_tick) = &self.old_tick_data {
            if old_tick == &tick.data {
                //log::debug!("Duplicate tick!");
                self.iqueue.clear();
                return;
            }
        }
        self.old_tick_data = Some(tick.data.clone());

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
                } else {
                    //need to realign
                    self.try_realign(tick.data);
                }
            },
            None => self.try_realign(tick.data),
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
    fn try_realign(&mut self, tick_data: ByteBuffer) {
        //try twice to realign the cipher, resetting in between
        //If the alignment fails twice the correct keystream must be > 100 million bytes in
        //How damn long do you have to be playing in the same area for that to be true
        if self.cipher.align_to_tick(tick_data.read_n_bytes_static(8).unwrap()) == false {
            self.cipher.reset();
            if self.cipher.align_to_tick(tick_data.read_n_bytes_static(8).unwrap()) == false {
                panic!("Cipher alignment panic! No suitable keystream could be found");
            }
        }
        let mut tick_data = ByteBuffer::new(self.cipher.apply_keystream_static(0, &tick_data.read_n_bytes_static(4).unwrap().to_vec()));
        self.current_tick = Some(tick_data.read_u32().unwrap());

        //Sometimes there are some extra packets left in the queue that the cipher cannot fully reverse to
        //Need to remove those packets
        while self.iqueue.iter().take(self.iqueue.len()-1).map(|x| x.data.rem_len()).sum::<usize>() > self.cipher.offset {
            let _ = self.iqueue.pop_front();
        }
        self.cipher.reverse(self.iqueue.iter().take(self.iqueue.len()-1).map(|x| x.data.rem_len()).sum::<usize>());
        self.drain_queue();
    }

    pub fn reset(&mut self) {
        self.cipher.reset();
        self.iqueue.clear();
        self.current_tick = None;
        self.old_tick_data = None;
    }
}