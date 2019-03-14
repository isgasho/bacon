#[macro_use] extern crate bacon;
use bacon::{ Bacon, BaconState, ciphers::{ Cipher, speck::Speck, Decrypt, Encrypt } };

use std::{
    io::{ Read, Write },
    net::{ TcpListener, TcpStream }
};


fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let post = b"POST / HTTP/1.1\r\n";
    
    let response = if buffer.starts_with(post) {

        let response = "HTTP/1.1 200 OK\r\n\r\n";
        let contents = "Christmas package received. Thanks for the bacon\n";
        format!("HTTP/1.1 200 OK\r\n\r\n{}", contents)
    } else {
        let response = "HTTP/1.1 405 OK\r\n\r\n";
        let contents = "We love Bacon, but you have to POST it.\n";
        format!("HTTP/1.1 200 OK\r\n\r\n{}", contents)
    };
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    println!("processing bacon");
    let mut bacon: Bacon = bincode::deserialize(&buffer).unwrap();
    dbg!(&bacon);
    let key_u128 = bacon::key_128("ufj6/ud.-%gdkfue"); // 17ZhjI3j/dshn3Kj
    let cipher: Speck = Speck::new(key_u128);
    bacon = cipher.decrypt(bacon);
    match unfry!(bacon, String) {
        Ok(s) => { 
            println!("We haved received the following message.");
            dbg!(s);
        },
        Err(e) => {
            dbg!(e);
            println!("This error probably originates from a wrong key.");
        }
    }
}

fn main() -> std::io::Result<()>  {
    let listener = TcpListener::bind("127.0.0.1:64100")?;
    println!("Server up and running.");
    // accept connections and process them serially
    for stream in listener.incoming() {
        println!("Connection established!");
        let msg = handle_client(stream?);

    }
    Ok(())
}