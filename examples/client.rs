#[macro_use] extern crate bacon;
extern crate bigint;
extern crate bincode;
use bacon::{ Bacon, BaconState, ciphers::{ Authenticate, Cipher, chacha20::ChaCha20, Encrypt, Nonce } };
use bigint::uint::U256;
use std::{ collections::HashMap, io::prelude::*, net::UdpSocket };

const BIND_ADDR: &str = "127.0.0.1:64101";


// $ cargo run --example client 127.0.0.1:64100 "Super secret message"
fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    // key
    let key: U256 = U256::from_dec_str("102853573294759285723534561345875635123503952762319857163587163501983275012378").unwrap();
    let cipher: ChaCha20 = ChaCha20::new(key, Nonce::BaconDefault);
    
    let mut bacon = Bacon::new(BaconState::Unfried, None, args[2].clone());
    bacon = cipher.encrypt(bacon);
    let mut buf: Vec<u8> = bincode::serialize(&bacon).unwrap();
    drop(bacon);

    // udp socket
    let mut socket = UdpSocket::bind(BIND_ADDR)?;
    match socket.send_to(&buf, "127.0.0.1:64100") {
        Ok(size) => {
            println!("{} bytes sent.", size);
        },
        _ => {}
    }
    // loop {
    //     let mut recv_buf = [0; 512];
    //     match socket.recv_from(&mut recv_buf) {
    //         Ok((amt, src)) => {
    //             println!("Receiving from {}", src);
    //             let bacon_package: BaconPackage = bincode::deserialize(&recv_buf).unwrap();
    //             dbg!(bacon_package);
    //             break;
    //         },
    //         _ => {}
    //     }
    // }
    Ok(())
}
