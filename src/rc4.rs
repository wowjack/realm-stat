/*
Custom rc4 implementation since crate ones wont work
*/
#![allow(unused_variables, dead_code)]

use byteorder::ByteOrder;

#[derive(Clone, Debug)]
pub struct Rc4 {
    key: Vec<u8>,
    state: [u8; 256],
    init_state: [u8; 256],
    i: i32,
    j: i32
}


impl Rc4 {
    /**
     * Construct a new Rc4 cipher from a byte vec key
     */
    pub fn new(key: Vec<u8>) -> Self {
        let mut state = [0u8; 256];
        for i in 0..=255 { state[i as usize] = i; }

        let mut j: usize = 0;
        for i in 0..256 {
            j = (j + state[i] as usize + key[i % key.len()] as usize) % 256;
            state.swap(i, j as usize);
        }

        Rc4 {
            key,
            state,
            init_state: state.clone(),
            i: 0,
            j: 0
        }
    }

    pub fn from_string_key(key: String) -> Self {
        //turn hex string into bytes then call new
        todo!()
    }

    pub fn skip(&mut self, amount: usize) {
        for k in 0..amount {
            self.i = (self.i + 1) % 256;
            self.j = (self.j + self.state[self.i as usize] as i32) % 256;
            self.state.swap(self.i as usize, self.j as usize);
        }
    }

    pub fn apply_keystream(&mut self, offset: usize, bytes: &Vec<u8>) -> Vec<u8> {
        bytes.clone().into_iter().take(offset).chain(bytes.iter().skip(offset).map(|b| *b ^ self.get_xor())).collect()
    }

    pub fn apply_keystream_static(&self, offset: usize, bytes: &Vec<u8>) -> Vec<u8> {
        self.clone().apply_keystream(offset, bytes)
    }

    fn get_xor(&mut self) -> u8 {
        self.skip(1);
        return self.state[(self.state[self.i as usize] as usize + self.state[self.j as usize] as usize) % 256]
    }

    pub fn reset(&mut self) {
        self.i = 0;
        self.j = 0;
        self.state = self.init_state.clone();
    }

    /**
     * Align the cipher's keystream such that decrypting chunk1, n bytes, then chunk 2 will yeild chunk1+1 = chunk2
     * returns the number that chunk2 decrypts to
     */
    pub fn align_to(&mut self, chunk1: &[u8], chunk2: &[u8], bytes_between: usize, extra: usize) -> u32 {
        log::debug!("Aligning cipher using {:?} {:?}", chunk1, chunk2);
        for i in 0..10_000_000 {
            let mut cipher = self.clone();
            cipher.skip(i);
            let c1 = byteorder::BigEndian::read_u32(&cipher.apply_keystream(0, &chunk1.to_vec()));
            cipher.skip(bytes_between);
            let c2 = byteorder::BigEndian::read_u32(&cipher.apply_keystream(0, &chunk2.to_vec()));

            if c1 + 1 == c2 {
                log::debug!("Found appropriate keystream: c1:{c1} c2:{c2} at offset {i}");
                cipher.skip(extra);
                *self = cipher;
                return c2;
            }
        }
        log::debug!("Failed to find cipher offset");
        return 0
    }

    pub fn align_to_real_tick(&mut self, real_tick: u32, encrypted_tick: &[u8], bytes_between: usize, extra: usize) -> bool {
        log::debug!("Real tick align {:?} to {real_tick}", encrypted_tick);
        for i in 0..(bytes_between + 100_000) {
            let mut cipher = self.clone();
            cipher.skip(4 + i);
            let decrypted_tick = byteorder::BigEndian::read_u32(&cipher.apply_keystream(0, &encrypted_tick.to_vec()));

            if decrypted_tick == real_tick {
                cipher.skip(extra);
                *self = cipher;
                log::debug!("Found appropriate offset {i} to align {:?} to {real_tick}", encrypted_tick);
                return true
            }
        }
        return false
    }
}