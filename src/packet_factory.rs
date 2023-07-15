use std::collections::VecDeque;

use byteorder::ByteOrder;
use etherparse::SlicedPacket;

use crate::{rc4::Rc4, byte_buffer::ByteBuffer, rotmg_packet::RotmgPacket};

const IKEY: [u8; 13] = [0xc9, 0x1d, 0x9e, 0xec, 0x42, 0x01, 0x60, 0x73, 0x0d, 0x82, 0x56, 0x04, 0xe0];
const OKEY: [u8; 13] = [0x5a, 0x4d, 0x20, 0x16, 0xbc, 0x16, 0xdc, 0x64, 0x88, 0x31, 0x94, 0xff, 0xd9];


/**
 * Input queue for payloads from sliced packets
 * Output queue for rotmg packets constructed from payload data
 * 
 * Stitches together chunks of application packets from tcp packet payloads.
 * Decrypts the application payload data and places the result in the output queue. 
 * Handles rc4 keystream desync using tick packets.
 */
pub struct RotmgPacketFactory {
    //byte queue for packet payloads
    iqueue: VecDeque<u8>,
    oqueue: VecDeque<RotmgPacket>,

    next_packet_len: u32, //number of bytes the input queue needs to construct a rotmg packet

    constructor: RotmgPacketConstructor
}
impl RotmgPacketFactory {
    pub fn new() -> Self {
        let iqueue = VecDeque::new();
        let oqueue = VecDeque::new();

        Self {
            iqueue,
            oqueue,

            next_packet_len: 0,

            constructor: RotmgPacketConstructor::new(),
        }
    }

    /**
     * Hand a sliced packet to the factory for processing
     */
    pub fn insert_packet(&mut self, packet: SlicedPacket) {
        //do nothing if the packet is empty
        if packet.payload.len() == 0 {return}

        //add packet data to the end of the queue
        self.iqueue.extend(packet.payload.iter());

        //check if application packets can be constructed
        self.check_queue()
    }


    /**
     * Read sliced packets from the input queue and stitch together payload data into rotmg packets
     */
    fn check_queue(&mut self) {
        loop {
            //Set the expected next packet length to the 4 bytes at the head of the input buffer
            if self.iqueue.len() > 3 {
                self.next_packet_len = byteorder::BigEndian::read_u32(&[self.iqueue[0], self.iqueue[1], self.iqueue[2], self.iqueue[3]]);
            }

            //break if the input queue isn't long enough to create another rotmg packet
            if self.iqueue.len() <= self.next_packet_len as usize {
                break
            }

            //Get the packet data from the head of the input queue
            let application_data: Vec<u8> = self.iqueue.drain(0..self.next_packet_len as usize).collect();
            //Try to construct a rotmg packet from the decrypted application data
            //If construction fails, check for packet errors and rc4 keystream desync
            match self.constructor.try_construct(application_data) {
                Ok(packet) => self.oqueue.push_back(packet),
                Err(e) => log::debug!("{:?}", e)
            }
            
        }
    }


    /**
     * Get a rotmg packet from the head of the output queue
     */
    pub fn get_packet(&mut self) -> Option<RotmgPacket> {
        self.oqueue.pop_front()
    }

    
}


/**
 * Takes in application packets that have been stitched together by the packet factory
 * Decrypts the packets and waits to output them
 * When a tick packet arrives, cipher alignment is checked and stored packets are sent out if the cipher is correct
 * 
 * Maintains cipher and tick alignment
 */
struct RotmgPacketConstructor {
    cipher: Rc4,
    current_tick: Option<u32>,
    //Encrypted tick packet, bytes received since the tick packet was received, cipher that would have been used to decrypt
    //This rc4 cipher is guaranteed to be behind the correct one 
    prev_tick_info: Option<(Vec<u8>, u32, Rc4)>,
}
impl RotmgPacketConstructor {
    fn new() -> Self {
        Self { cipher: Rc4::new(Vec::from(IKEY)), current_tick: None, prev_tick_info: None }
    }

    /**
     * Deserialize the packet and convert into rotmg packet enum 
     * 
     * If the packet is a tick packet, check correctness and increment tick counter
     */
    fn try_construct(&mut self, data: Vec<u8>) -> Result<RotmgPacket, ()> {
        let old_cipher = self.cipher.clone();
        let plaintext = self.cipher.apply_keystream(5, &data);
        let buf = ByteBuffer::new(plaintext.clone());
        let new_packet = match RotmgPacket::try_from(buf) {
            Ok(e) => e,
            Err(()) => {
                log::debug!("Error constructing packet: {:?}", plaintext);
                return Err(())
            }
        };

        if let RotmgPacket::Reconnect {..}  = new_packet {
            //log::debug!("{:?}", new_packet);
            self.reset()
        } else if let RotmgPacket::NewTick {..} = new_packet.clone() {
            log::debug!("{:?}", new_packet);
            self.process_tick(data.clone(), new_packet.clone(), old_cipher);
        } else {
            //Add to byte counter in prev_tick_info
            if let Some((_, num_bytes, _)) = &mut self.prev_tick_info {
                *num_bytes += (data.len() - 5) as u32;
            }
        }

        //log::debug!("{:?}", new_packet);
        
        
                
        return Ok(new_packet)
    }

    /**
     * Save some info about the packet if it is a tick packet
     * This is useful for realigning the cipher when it breaks
     */
    fn process_tick(&mut self, tick_data: Vec<u8>, packet: RotmgPacket, cipher: Rc4) {
        if let RotmgPacket::NewTick { tick_id, .. } = packet {
            self.current_tick = match self.current_tick {None=>None, Some(n)=>Some(n+1)};

            if let Some(t) = self.current_tick {
                if t == tick_id {
                    //alignment is all good
                    self.prev_tick_info = Some((tick_data, 0, cipher));
                    return
                } else {
                    //Something happened with packet decryption, realign cipher
                    //First try to just tweak the alignment of the cipher to get the new tick id to match
                    //If it does not align within a reasonable search space, then reset and brute force search
                    log::debug!("Tick packet alignment failure. Expected {t} got {tick_id}");

                    self.realign(tick_data.clone());
                    self.prev_tick_info = Some((tick_data, 0, cipher));
                    return
                }
            } else {
                if let None = self.prev_tick_info {
                    //curent tick and prev_tick info are unset, first tick packet seen?
                    log::debug!("First tick packet seen. Saving info");
                    if tick_id == 0 {
                        log::debug!("First tick packet is 0! Alignment is good!");
                        self.current_tick = Some(0);
                    }
                    self.prev_tick_info = Some((tick_data, 0, cipher));
                    return
                } else {
                    //prev tick info is set but not current tick, second tick packet seen?
                    log::debug!("Second tick packet seen. Aligning");
                    self.realign(tick_data.clone());
                    self.prev_tick_info = Some((tick_data, 0, cipher));
                    return
                }
            }
        }
    }

    /**
     * Attempts to realign the rc4 cipher and current tick counter using the stored previous tick packet.
     * 
     * If self.prev_tick_packet == None, tick packet is the first tick packet so it is incorrectly decrypted unless it is 0
     */
    fn realign(&mut self, tick_data: Vec<u8>) {
        
        if let Some((old_bytes, bytes_between, cipher)) = &mut self.prev_tick_info {

            log::debug!("Realigning cipher");
            let c1: Vec<u8> = old_bytes.clone().into_iter().skip(5).take(4).collect();
            let c2: Vec<u8> = tick_data.clone().into_iter().skip(5).take(4).collect();

            //First try to realign the cipher by trying a bunch of different offsets for the second tick until key^second_tick = self.current_tick 
            if let Some(t) = self.current_tick {
                let mut new_cipher = cipher.clone();
                let result = new_cipher.align_to_real_tick(t, &c2, *bytes_between as usize + old_bytes.len()-9, tick_data.len()-9);
                if result == true {
                    self.cipher = new_cipher;
                    return
                }
            }

            if c1 == c2 {
                log::debug!("old and new cipher are the same, resetting");
                self.reset();
                return;
            }
            
            self.cipher.reset();
            self.current_tick = Some(self.cipher.align_to(&c1, &c2, *bytes_between as usize + old_bytes.len() - 9, tick_data.len() - 9));
        }
    }

    fn reset(&mut self) {
        self.cipher.reset();
        self.current_tick = None;
        self.prev_tick_info = None;
    }
}


#[derive(Debug)]
enum RotmgPacketConstructError {
    TickError,
    BufferSizeError
}