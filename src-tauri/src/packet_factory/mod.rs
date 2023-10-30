pub mod rotmg_packet;
pub mod byte_buffer;
pub mod data_types;
mod rotmg_packet_stitcher;
mod tick_frame_constructor;
mod rotmg_packet_decryptor;

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use etherparse::SlicedPacket;
use self::rotmg_packet::RotmgPacket;
use self::rotmg_packet_decryptor::RotmgPacketDecryptor;
use self::rotmg_packet_stitcher::RotmgPacketStitcher;
use self::tick_frame_constructor::TickFrameConstructor;

/*
    Consider that memory issues may arrive if the factory oqueue gets too large when capturing for a long time
    or working with large pcap files. 
*/


/// Stitches together, validates, and decrypts packets \
/// This might be massively over engineered
pub struct RotmgPacketFactory {
    /// Object to get raw packet payloads and stitch them together into encrypted ROTMG packet data chunks
    pub stitcher: RotmgPacketStitcher,

    /// Groups statched packets together into frames ending with a tick
    tick_frame_constructor: TickFrameConstructor,

    /// Takes in frames of packets from the frame construcor and attempts to decrypt them
    _decrypter: Arc<Mutex<RotmgPacketDecryptor>>,

    /// Output queue of fully decrypted and constructed ROTMG packets
    ///* Wrapped in an arc and mutex since the constructor's pipe closure needs to capture it to send packets here
    oqueue: Arc<Mutex<VecDeque<RotmgPacket>>>
}
impl RotmgPacketFactory {
    pub fn new() -> Self {
        let stitcher = RotmgPacketStitcher::new();
        let oqueue = Arc::new(Mutex::new(VecDeque::new()));

        let pipe_oqueue = oqueue.clone();
        let _decrypter = Arc::new(Mutex::new(RotmgPacketDecryptor::new(Box::new(
            move |packets: Vec<RotmgPacket>| {
                for packet in packets.iter() {
                    //println!("{:?}", packet);
                }
                pipe_oqueue.lock().unwrap().extend(packets.into_iter());
            }
        ))));

        let tick_pipe_decrypter = _decrypter.clone();
        let tick_frame_constructor = TickFrameConstructor::new(Box::new(
            move |frame| tick_pipe_decrypter.lock().unwrap().insert_frame(frame)
        ));

        Self { stitcher, tick_frame_constructor, _decrypter, oqueue }
    }

    /// Hand a sliced packet to the factory for processing
    pub fn insert_packet(&mut self, packet: SlicedPacket) {
        // Send packet to the stitcher, then send resulting stitched packets to the frame constructor
        // The frame constructor will handle the rest using its pipe
        self.stitcher
            .insert_packet(packet.payload)
            .into_iter()
            .for_each(|sp| self.tick_frame_constructor.insert_packet(sp));
    }


    
    /// Get a rotmg packet from the head of the output queue
    pub fn get_packet(&mut self) -> Option<RotmgPacket> {
        self.oqueue.lock().unwrap().pop_front()
    }

    /// Get all the rotmg packets from the the output queue
    pub fn get_packets(&mut self) -> Vec<RotmgPacket> {
        let mut oqueue = self.oqueue.lock().unwrap();
        let len = oqueue.len();
        oqueue.drain(0..len).collect()
    }

    /// Completely clear and reset the packet factory
    ///* Reset the packet stitcher
    ///* Reset the tick frame constructor
    ///* Reset the decrypter
    ///* Clear the output queue
    pub fn reset(&mut self) {
        self.stitcher.reset();
        self.tick_frame_constructor.reset();
        self._decrypter.lock().unwrap().reset();
        self.oqueue.lock().unwrap().clear();
    }
}





