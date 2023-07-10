/*
Custom rc4 implementation since crate ones wont work
*/
#![allow(unused_variables, dead_code)]

#[derive(Clone)]
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

    pub fn apply_keystream(&mut self, offset: usize, bytes: &mut [u8]) {
        for byte in bytes.iter_mut().skip(offset) {
            *byte = *byte ^ self.get_xor();
        }
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
}