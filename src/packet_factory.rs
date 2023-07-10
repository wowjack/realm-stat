use std::{collections::VecDeque, thread::{self, JoinHandle}, sync::{Mutex, Arc, Condvar}};
use log::{info, debug};

use byteorder::ByteOrder;
use etherparse::SlicedPacket;

use crate::{rotmg_packet_type::RotmgPacketType, rc4::Rc4};


/**
 * Input queue for sliced packets
 * Output queue for rotmg packets
 * 
 */
pub struct RotmgPacketFactory {
    //byte queue for packet payloads
    iqueue: VecDeque<u8>,
    oqueue: VecDeque<RotmgPacket>,

    next_packet_len: u32, //number of bytes the input queue needs to construct a rotmg packet

    cipher: Rc4
}
impl RotmgPacketFactory {
    pub fn new() -> Self {
        let iqueue = VecDeque::new();
        let oqueue = VecDeque::new();

        Self {
            iqueue,
            oqueue,

            next_packet_len: 0,

            cipher: Rc4::new(vec![0xc9, 0x1d, 0x9e, 0xec, 0x42, 0x01, 0x60, 0x73, 0x0d, 0x82, 0x56, 0x04, 0xe0]),
        }
    }

    pub fn insert_packet(&mut self, packet: SlicedPacket) {
        //do nothing if the packet is empty
        if packet.payload.len() == 0 {return}

        //add packet data to the end of the queue
        //if the queue is empty, read the leading u32
        if self.iqueue.len() == 0 {
            self.next_packet_len = byteorder::BigEndian::read_u32(packet.payload);
        }
        self.iqueue.extend(packet.payload.iter());

        self.check_queue()
    }


    //Extract rotmg packets from the input queue until there are no more
    fn check_queue(&mut self) {
        loop {
            //check if queue is large enough to create another rotmg packet
            if self.iqueue.len() <= self.next_packet_len as usize {
                break
            }

            //Create the new rotmg packet and push to the output queue
            let mut new_packet: Vec<u8> = self.iqueue.drain(0..self.next_packet_len as usize).collect();

            //log::error!("Extracted: {:?}\nRemaining: {:?}", new_packet, self.iqueue.clone().into_iter().collect::<Vec<u8>>());

            self.cipher.apply_keystream(5, &mut new_packet);
            self.oqueue.push_back(RotmgPacket { leading_int: self.next_packet_len, packet_type: new_packet[4], payload: new_packet });

            //reset the next_packet_len if the queue still has items
            if self.iqueue.len() > 0 {
                if self.iqueue.len() < 5 {
                    panic!("Received unusual packet: {:?}", self.iqueue.as_slices());
                }
                self.next_packet_len = byteorder::BigEndian::read_u32(&[self.iqueue[0], self.iqueue[1], self.iqueue[2], self.iqueue[3]])
            }
        }
    }

    pub fn get_packet(&mut self) -> Option<RotmgPacket> {
        self.oqueue.pop_front()
    }

    
}

pub struct RotmgPacket {
    pub leading_int: u32,
    pub packet_type: u8,
    pub payload: Vec<u8>
}