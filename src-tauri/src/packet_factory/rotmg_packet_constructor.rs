use std::{collections::VecDeque, sync::{Mutex, Arc}};
use byteorder::{BigEndian, ByteOrder};
use crate::rc4::Rc4;
use super::{rotmg_packet::RotmgPacket, byte_buffer::ByteBuffer, rotmg_packet_stitcher::StitchedPacket};


const IKEY: [u8; 13] = [0xc9, 0x1d, 0x9e, 0xec, 0x42, 0x01, 0x60, 0x73, 0x0d, 0x82, 0x56, 0x04, 0xe0];
const _OKEY: [u8; 13] = [0x5a, 0x4d, 0x20, 0x16, 0xbc, 0x16, 0xdc, 0x64, 0x88, 0x31, 0x94, 0xff, 0xd9];

type ConstructorPipe = Box<dyn Fn(Vec<RotmgPacket>) + Send + Sync + 'static>;
/**
 * Takes in application packets that have been stitched together by the packet factory
 * Decrypts the packets and waits to output them
 * When a tick packet arrives, cipher alignment is checked and stored packets are sent out if the cipher is correct
 * 
 * Maintains cipher and tick alignment
 */
pub struct RotmgPacketConstructor {
    /// Queue of incoming packets that are still encrypted
    iqueue: VecDeque<StitchedPacket>,

    /// Closure where packets are sent after they are properly decrypted and constructed
    pipe: ConstructorPipe,

    ///Cipher that should be correctly aligned to decrypt the packets in the iqueue
    pub cipher: Rc4,
    
    ///The previously detected tick number along with the cipher state after decrypting that previous tick packet
    prev_tick_data: Option<(u32, Rc4)>
}
impl RotmgPacketConstructor {
    /// Construct a new RotmgPacketConstruct object that will send fully processed packets to the closure arg
    pub fn new(pipe: ConstructorPipe) -> Self {
        Self {
            iqueue: VecDeque::with_capacity(10),
            pipe,
            cipher: Rc4::new(Vec::from(IKEY)),
            prev_tick_data: None,
        }
    }

    /// Give a packet to the constructor for decrypting and construction
    ///* I don't think theres a need to insert multiple packets at once, realistically the stitcher will just produce one packet at a time
    pub fn insert_packet(&mut self, packet: StitchedPacket) {
        self.iqueue.push_back(packet.clone());

        if packet.type_num == 10 { // Got a tick packet
            // Decrypt all the packets in the queue and perform a sanity check on the final decrypted tick packet
        }

        todo!()
    }

    /// Completely reset the packet constructor object
    ///* Clear the input queue
    ///* Reset the cipher its initial state
    ///* Clear previous tick data
    ///* The output pipe is kept
    pub fn reset(&mut self) {
        self.iqueue.clear();
        self.cipher.reset();
        self.prev_tick_data = None;
    }
}

