extern crate bacon;
extern crate bincode;

use bacon::{ Bacon };
use std::{ net::UdpSocket };

const BIND_ADDR: &str = "127.0.0.1:64100";
fn main() -> std::io::Result<()> {
    let mut socket = UdpSocket::bind(BIND_ADDR)?;
    println!("Server up and running at 127.0.0.1:64100");
    let mut buf = [0; 512];
    loop {
        match socket.recv_from(&mut buf) {
            Ok((amt, src)) => {
                println!("Receiving from {:?}", src);
                //println!("{}", String::from_utf8_lossy(&buf[..]) );
                let bacon: Bacon = bincode::deserialize(&buf).unwrap();
                dbg!(bacon);
                
            },
            Err(e) => { dbg!(e); }      
        }
    }
    Ok(())
}