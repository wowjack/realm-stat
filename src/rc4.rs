/*
Custom rc4 implementation since crate ones wont work
*/
#![allow(unused_variables, dead_code)]

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
     * Iterate through the keystream until chunk1^key + 1 = chunk2^(key.skip(bytes_between))
     * 
     * Ultimately the cipher ends with an offset directly before correctly encrypting chunk1
     */
    pub fn align_to(&mut self, chunk1: &[u8], chunk2: &[u8], bytes_between: usize) -> u32 {
        log::debug!("Aligning cipher using {:?} {:?}", chunk1, chunk2);
        let mut new_cipher = self.clone();
        for i in 0..10_000_000 {
            let mut tmp_cipher = new_cipher.clone();
            let r1 = tmp_cipher.apply_keystream(0, &chunk1.to_vec());
            let r1 = u32::from_be_bytes([r1[0], r1[1], r1[2], r1[3]]);
            tmp_cipher.skip(bytes_between);
            let r2 = tmp_cipher.apply_keystream(0, &chunk2.to_vec());
            let r2 = u32::from_be_bytes([r2[0], r2[1], r2[2], r2[3]]);
            if r1 + 1 == r2 {
                *self = new_cipher;
                return r2
            }

            new_cipher.skip(1);
        }
        log::debug!("Failed to find cipher offset");
        return u32::MAX;
    }

    /**
     * Iterate through the keystream until encrypted_tick^key = real_tick
     * 
     * Ultimately the cipher ends up with an offset such that applying the keystream to the encrypted tick will yeild the real tick
     */
    pub fn align_to_real_tick(&mut self, real_tick: u32, encrypted_tick: &[u8]) -> bool {
        log::debug!("Real tick align: {:?} to {real_tick}", encrypted_tick);
        //We want to find this sequence of u8s in the keystream
        let xor_key = (u32::from_be_bytes([encrypted_tick[0], encrypted_tick[1], encrypted_tick[2], encrypted_tick[3]]) ^ real_tick).to_be_bytes();
        let mut new_cipher = self.clone();
        for i in 0..(100_000) {
            let mut tmp_cipher = new_cipher.clone();
            if tmp_cipher.get_xor() == xor_key[0] {
                if tmp_cipher.get_xor() == xor_key[1] {
                    if tmp_cipher.get_xor() == xor_key[2] {
                        if tmp_cipher.get_xor() == xor_key[3] {
                            *self = new_cipher;
                            log::debug!("Found appropriate keystream chunk at offset {}", i+4);
                            return true;
                        }
                    }
                }
            }
            new_cipher.skip(1)
        }
        return false
    }
}