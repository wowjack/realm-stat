use crate::sniffer::Sniffer;

mod rc4;
mod packet_factory;
mod sniffer;

fn main() {
    let _ = simple_logging::log_to_file("log.log", log::LevelFilter::Debug);

    let mut sniffer = Sniffer::ask_for_device();
    sniffer.start();
    //sniffer.start_using_file("./sample/rotmg.pcap".into());

    /*
    let mut cipher = rc4::Rc4::new(vec![0xc9, 0x1d, 0x9e, 0xec, 0x42, 0x01, 0x60, 0x73, 0x0d, 0x82, 0x56, 0x04, 0xe0]);
    let mut tmp_cipher = cipher.clone();
    tmp_cipher.skip(99_000_000);

    let tick = [0u8, 0, 0, 234, 0, 0, 0, 201];
    let ciphertext = tmp_cipher.apply_keystream_static(0, &tick.to_vec());
    let key = tmp_cipher.apply_keystream(0, &vec![0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]);

    log::debug!("ciphertext: {:?}", ciphertext);
    log::debug!("key:        {:?}", key);

    cipher.align_to_tick_experimental(&ciphertext);
    let plaintext = cipher.apply_keystream(0, &ciphertext);
    log::debug!("Plaintext: {:?}", plaintext);
    */
    
}