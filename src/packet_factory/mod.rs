pub mod rotmg_packet;
pub mod byte_buffer;
mod rotmg_packet_constructor;
mod rotmg_packet_stitcher;

use etherparse::SlicedPacket;
use self::rotmg_packet::RotmgPacket;
use self::rotmg_packet_constructor::RotmgPacketConstructor;
use self::rotmg_packet_stitcher::RotmgPacketStitcher;





/**
 * Stitches together, validates, and decrypts packets
 */
pub struct RotmgPacketFactory {
    stitcher: RotmgPacketStitcher,
    constructor: RotmgPacketConstructor
}
impl RotmgPacketFactory {
    pub fn new() -> Self {
        Self {
            stitcher: RotmgPacketStitcher::new(),
            constructor: RotmgPacketConstructor::new(),
        }
    }

    /**
     * Hand a sliced packet to the factory for processing
     */
    pub fn insert_packet(&mut self, packet: SlicedPacket) {
        //do nothing if the packet is empty
        if packet.payload.len() == 0 {return}

        //send packet to the stitcher
        self.stitcher.insert_packet(packet.payload);

        //get any packets output by the stitcher and send them to the constructor
        while let Some(p) = self.stitcher.get_packet() {
            self.constructor.insert_packet(p);
        }
    }


    /**
     * Get a rotmg packet from the head of the output queue
     */
    pub fn get_packet(&mut self) -> Option<RotmgPacket> {
        self.constructor.get_packet()
    }

    
}



