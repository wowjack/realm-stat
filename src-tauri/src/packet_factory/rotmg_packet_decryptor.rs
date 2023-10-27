use crate::rc4::Rc4;

use super::{rotmg_packet::RotmgPacket, tick_frame_constructor::TickFrame};

const IKEY: [u8; 13] = [0xc9, 0x1d, 0x9e, 0xec, 0x42, 0x01, 0x60, 0x73, 0x0d, 0x82, 0x56, 0x04, 0xe0];
const _OKEY: [u8; 13] = [0x5a, 0x4d, 0x20, 0x16, 0xbc, 0x16, 0xdc, 0x64, 0x88, 0x31, 0x94, 0xff, 0xd9];

type PacketDecryptorPipe = Box<dyn Fn(Vec<RotmgPacket>) + Send + Sync + 'static>;


/*
    I'm thinking spawn a worker thread that will grab frames from a queue and attempt to decrypt them one at a time.
    If the worker needs to spend a while trying to align the cipher it will not block the capture thread.
    Use condition variable so the worker doesn't spin if the queue is empty because nothing is being captured.
    Need to consider the case where the factory is reset but the worker is still aligning the cipher.
    Perhaps use message passing channel to send the thread a message whenever the factory is reset.
    After successfully decrypting a frame, the worker will check for a reset message and wont pipe the decrypted packets out if reset was received.
    This shouldn't be a problem if the factory is reset, then started up again quickly. As the worker is working, the capture thread can easily clear
    the queue and send the reset message. Any incoming frames after that will enter the queue. The worker will finish working on the frame,
    drop the old packets, then continue on working with the new frames in the queue. 
*/

pub struct RotmgPacketDecryptor {
    cipher: Rc4,
    pipe: PacketDecryptorPipe,
    prev_tick: Option<RotmgPacket>
}
impl RotmgPacketDecryptor {
    pub fn new(pipe: PacketDecryptorPipe) -> Self {
        Self {
            cipher: Rc4::new(Vec::from(IKEY)),
            pipe,
            prev_tick: None
        }
    }

    /// Attempts to decrypt the frame of packets \
    /// Performs a sanity check on the decrypted packets to make sure everything is okay
    pub fn insert_frame(&mut self, frame: TickFrame) {
        log::debug!("Got frame of packets");
    }

    pub fn reset(&mut self) {
        self.cipher.reset();
        self.prev_tick = None;
    }
}