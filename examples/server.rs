#[macro_use]
extern crate bacon;
extern crate bincode;

use bacon::{ Bacon, ciphers::{ chacha20::ChaCha20, Cipher, Decrypt, Nonce } };
use bigint::uint::U256;
use std::{ net::UdpSocket };

const BIND_ADDR: &str = "127.0.0.1:64100";
fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind(BIND_ADDR)?;
    println!("Server up and running at 127.0.0.1:64100");
    let mut buf: [u8; 512] = [0; 512];
    loop {
        match socket.recv_from(&mut buf) {
            Ok((_, src)) => {
                println!("Receiving from {:?}", src);
                //println!("{}", String::from_utf8_lossy(&buf[..]) );
                // encrypted bacon
                let mut bacon: Bacon = bincode::deserialize(&buf).unwrap();
                let key: U256 = U256::from_dec_str("102853573294759285723534561345875635123503952762319857163587163501983275012378").unwrap();
                dbg!(&bacon);
                let cipher = ChaCha20::new(key, Nonce::BaconDefault);
                bacon = cipher.decrypt(bacon);
                let s = unfry!(bacon, String).unwrap();
                dbg!(s);
                
            },
            Err(e) => { dbg!(e); }      
        }
    }
    Ok(())
}