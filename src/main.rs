use packet_factory::byte_buffer::ByteBuffer;

use crate::sniffer::Sniffer;

mod rc4;
mod packet_factory;
mod sniffer;

fn main() {
    let _ = simple_logging::log_to_file("log.log", log::LevelFilter::Debug);

    let mut sniffer = Sniffer::ask_for_device();
    sniffer.start();

    // let real_tick = 53u32;    
    // let mut cipher  = rc4::Rc4::new(vec![0xc9, 0x1d, 0x9e, 0xec, 0x42, 0x01, 0x60, 0x73, 0x0d, 0x82, 0x56, 0x04, 0xe0]);

    // let mut tmp_cipher = cipher.clone();
    // tmp_cipher.skip(1_000_001);
    // let encrypted_tick1 = tmp_cipher.apply_keystream(0, &real_tick.to_be_bytes().to_vec());
    // tmp_cipher.skip(1000);
    // let encrypted_tick2 = tmp_cipher.apply_keystream(0, &(real_tick+1).to_be_bytes().to_vec());

    // let res = cipher.align_to(&encrypted_tick1, &encrypted_tick2, 1000);

    // if res != 0 {
    //     log::debug!("Success")
    // } else {
    //     log::debug!("Fail")
    // }



    /*
    let mut cipher = rc4::Rc4::new(vec![0xc9, 0x1d, 0x9e, 0xec, 0x42, 0x01, 0x60, 0x73, 0x0d, 0x82, 0x56, 0x04, 0xe0]);
    let mut test_cipher = cipher.clone();

    cipher.skip(49);


    let c1 = cipher.apply_keystream(0, &vec![0x0, 0x0, 0x0, 0x3]);
    cipher.skip(22);
    let c2 = cipher.applyeystream(0, &vec![0x0, 0x0, 0x0, 0x4]);
_k
    println!("{:?} {:?}", c1, c2);

    test_cipher.align_to(&c1, &c2, 22);
    */
}