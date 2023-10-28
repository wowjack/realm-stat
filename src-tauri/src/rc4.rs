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
    j: i32,
    pub offset: usize
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
            j: 0,
            offset: 0
        }
    }

    pub fn from_string_key(key: String) -> Self {
        //turn hex string into bytes then call new
        todo!()
    }

    pub fn skip(&mut self, amount: usize) {
        self.offset += amount;
        for k in 0..amount {
            self.i = (self.i + 1) % 256;
            self.j = (self.j + self.state[self.i as usize] as i32) % 256;
            self.state.swap(self.i as usize, self.j as usize);
        }
    }

    pub fn apply_keystream(&mut self, offset: usize, bytes: &Vec<u8>) -> Vec<u8> {
        bytes.clone().into_iter().take(offset).chain(bytes.iter().skip(offset).map(|b| *b ^ self.get_keystream_byte())).collect()
    }

    pub fn apply_keystream_static(&self, offset: usize, bytes: &Vec<u8>) -> Vec<u8> {
        self.clone().apply_keystream(offset, bytes)
    }

    fn get_keystream_byte(&mut self) -> u8 {
        self.skip(1);
        return self.state[(self.state[self.i as usize] as usize + self.state[self.j as usize] as usize) % 256]
    }

    fn get_n_keystream_bytes_static(&self, amount: usize) -> Vec<u8> {
        let mut tmp_cipher = self.clone();
        (0..amount).map(|_| tmp_cipher.get_keystream_byte()).collect()
    }

    pub fn reset(&mut self) {
        self.i = 0;
        self.j = 0;
        self.state = self.init_state.clone();
        self.offset = 0;
    }

    pub fn reverse(&mut self, amount: usize) {
        if amount > self.offset {
            panic!("cannot reverse rc4 greater than offset");
        }
        let offset = self.offset - amount;
        self.reset();
        self.skip(offset);
    }

    /**
     * Makes some reasonable assumptions about tick packets to align the rc4 cipher.
     * A massive improvement from the previous version, this method requires no other tick packet or known tick id to align.
     * 
     * If the tick id is greater than 65_535 this method will fail to align the cipher.
     * i.e. the player has been in the same area for > 3.64 hours.
     * 
     * If the tick time is greater than 255 this method will fail to align the cipher.
     * I have not witnessed this happen myself, so it seems pretty uncommon. Either way if it fails it will just try again on the next tick packet.
     * 
     * If the real cipher offset is more than 100 million past the current offset this, method will fail to align the rc4 cipher.
     * With very minimal testing I think it takes about an hour of activity in one area to reach 100 million.
     */
    pub fn align_to_tick(&mut self, tick_data: &[u8]) -> bool {
        //If the proper keystream is within 100 million bytes of the current cipher offset, it will be found
        //Perhaps use itertools windows to improve search using composition

        for _ in 0..100_000_000 {
            let window = self.get_n_keystream_bytes_static(7);
            if window[0] == tick_data[0] &&
                window[1] == tick_data[1] &&
                window[4] == tick_data[4] &&
                window[5] == tick_data[5] &&
                window[6] == tick_data[6] 
            {
                return true;
            }
            self.skip(1);
        }
        //log::debug!("Failed to find keystream");
        return false;
    }
}
