use crate::rc4::Rc4;

use super::{rotmg_packet::RotmgPacket, tick_frame_constructor::EncryptedTickFrame, rotmg_packet_stitcher::StitchedPacket};

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
    pipe: PacketDecryptorPipe,
    prev_tick: Option<PrevTickData>
}
impl RotmgPacketDecryptor {
    pub fn new(pipe: PacketDecryptorPipe) -> Self {
        Self {
            pipe,
            prev_tick: None
        }
    }

    /// Attempts to decrypt the frame of packets \
    /// Performs a sanity check on the decrypted packets to make sure everything is okay
    pub fn insert_frame(&mut self, frame: EncryptedTickFrame) {
        let (decrypted_frame, _integrity) = self.decrypt(frame);
        match _integrity {
            DecryptionIntegrity::Ok => (),
            DecryptionIntegrity::MissingBytes(n) => log::debug!("Missing bytes: {n}"),
            DecryptionIntegrity::ExtraBytes(n) => log::debug!("Extra bytes: {n}"),
            DecryptionIntegrity::Warn => log::debug!("Warn"),
            DecryptionIntegrity::Duplicate => log::debug!("Duplicate"),
        }
        (self.pipe)(decrypted_frame.to_rotmg_packets());
    }

    /// Takes in an encrypted tick frame and tries its best to decrypt it.
    fn decrypt(&mut self, frame: EncryptedTickFrame) -> (DecryptedTickFrame, DecryptionIntegrity) {
        match self.prev_tick.clone() {
            Some(data) =>  self.decrypt_with_prev_tick(frame, data),
            None => (self.decrypt_without_prev_tick(frame), DecryptionIntegrity::Warn)
        }
    }


    fn decrypt_without_prev_tick(&mut self, frame: EncryptedTickFrame) -> DecryptedTickFrame {
        let mut cipher = Rc4::new(IKEY.to_vec());
        cipher.align_to_tick(frame.terminating_tick.buffer.read_n_bytes_static(7).unwrap());
        cipher.reverse(frame.payload_len());

        let encrypted_tick = frame.terminating_tick.clone();
        let decrypted_frame = frame.decrypt(&mut cipher);
        let decrypted_tick = decrypted_frame.terminating_tick.clone();
        self.prev_tick = Some(PrevTickData { encrypted_data: encrypted_tick, decrypted_data: decrypted_tick, cipher: cipher });
        return decrypted_frame
    }

    fn decrypt_with_prev_tick(&mut self, frame: EncryptedTickFrame, mut prev_tick_data: PrevTickData) -> (DecryptedTickFrame, DecryptionIntegrity) {
        if frame.terminating_tick.buffer.read_n_bytes_static(4).unwrap_or(&[0,0,0,0]) == prev_tick_data.encrypted_data.buffer.read_n_bytes_static(4).unwrap_or(&[0,0,0,0]) {
            // Got duplicate tick
            //log::debug!("Got duplicate tick frame");
            //log::debug!("{:?}\n{:?}\n", prev_tick_data.encrypted_data, frame.terminating_tick);
            return (frame.decrypt(&mut prev_tick_data.cipher), DecryptionIntegrity::Duplicate);
        }
        let mut cipher = prev_tick_data.cipher.clone();
        cipher.skip(frame.payload_len());
        let mut new_tick = frame.terminating_tick.clone();
        new_tick.decrypt(&mut cipher);
        if RotmgPacketDecryptor::validate_ticks(&prev_tick_data.decrypted_data, &new_tick) == false {
            let frame_len = frame.total_len();
            let decrypted_tick_frame = self.decrypt_without_prev_tick(frame);
            let new_cipher = &self.prev_tick.as_ref().unwrap().cipher;
            if new_cipher.offset < prev_tick_data.cipher.offset + frame_len {
                return (decrypted_tick_frame, DecryptionIntegrity::MissingBytes(prev_tick_data.cipher.offset + frame_len - new_cipher.offset))
            } else if new_cipher.offset > prev_tick_data.cipher.offset + frame_len {
                return (decrypted_tick_frame, DecryptionIntegrity::ExtraBytes(new_cipher.offset - prev_tick_data.cipher.offset - frame_len))
            }
            return (decrypted_tick_frame, DecryptionIntegrity::Warn)
        }
        let mut new_cipher = prev_tick_data.cipher.clone();
        let decrypted_frame = frame.decrypt(&mut new_cipher);

        self.prev_tick = Some(PrevTickData { encrypted_data: frame.terminating_tick, decrypted_data: new_tick, cipher: cipher });
        return (decrypted_frame, DecryptionIntegrity::Ok)
    }

    fn validate_ticks(tick1: &StitchedPacket, tick2: &StitchedPacket) -> bool {
        tick1.buffer.read_u32_static().unwrap_or(0) + 1 == tick2.buffer.read_u32_static().unwrap_or(0)
    }

    pub fn reset(&mut self) {
        self.prev_tick = None;
    }
}

#[derive(Clone)]
struct PrevTickData {
    encrypted_data: StitchedPacket,
    decrypted_data: StitchedPacket,
    
    /// Cipher aligned directly after decrypting the associated tick packet
    cipher: Rc4
}


pub struct DecryptedTickFrame {
    pub packets: Vec<StitchedPacket>,
    pub terminating_tick: StitchedPacket,
}
impl DecryptedTickFrame {
    pub fn to_rotmg_packets(self) -> Vec<RotmgPacket> {
        self.packets
            .into_iter()
            .chain([self.terminating_tick].into_iter())
            .filter_map(|sp| RotmgPacket::try_from(sp).ok())
            .collect()
    }
}


pub enum DecryptionIntegrity {
    Ok,
    MissingBytes(usize),
    ExtraBytes(usize),
    Warn,
    Duplicate
}