use crate::sniffer::Sniffer;

mod rc4;
mod packet_factory;
mod sniffer;

fn main() {
    let _ = simple_logging::log_to_file("log.log", log::LevelFilter::Debug);

    let mut sniffer = Sniffer::ask_for_device();
    //sniffer.start();
    sniffer.start_using_file("./sample/rotmg.pcap".into());


    /*
    let mut cipher = rc4::Rc4::new(vec![0xc9, 0x1d, 0x9e, 0xec, 0x42, 0x01, 0x60, 0x73, 0x0d, 0x82, 0x56, 0x04, 0xe0]);
    let mut new_cipher = cipher.clone();
    new_cipher.skip(10_255);

    let num: u32 = 100;
    let bytes_between: usize = 4001;
    let encrypted1 = new_cipher.apply_keystream(0, &num.to_be_bytes().into());
    new_cipher.skip(bytes_between);
    let encrypted2 = new_cipher.apply_keystream(0, &(num+1).to_be_bytes().into());

    cipher.align_to(&encrypted1, &encrypted2, bytes_between);

    let decrypted1 = cipher.apply_keystream(0, &encrypted1);
    cipher.skip(bytes_between);
    let decrypted2 = cipher.apply_keystream(0, &encrypted2);

    println!("{:?} {:?}", u32::from_be_bytes([decrypted1[0], decrypted1[1], decrypted1[2], decrypted1[3]]), u32::from_be_bytes([decrypted2[0], decrypted2[1], decrypted2[2], decrypted2[3]]));
    */
    
}