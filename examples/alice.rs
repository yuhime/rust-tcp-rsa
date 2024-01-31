use rand;
use rsa::pkcs8::DecodePublicKey;
use rsa::{Pkcs1v15Encrypt, RsaPublicKey};
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:9123").unwrap();
    let mut buf = [0; 294];
    let mut rng = rand::thread_rng();
    // let mut buf = Vec::new();
    stream.read(&mut buf).unwrap();

    let pub_key = RsaPublicKey::from_public_key_der(&buf).unwrap();

    let mut message = String::new();
    loop {
        print!("msg : ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut message).unwrap();
        message.trim().to_string();
        let enc_msg = pub_key
            .encrypt(&mut rng, Pkcs1v15Encrypt, message.as_bytes())
            .unwrap();

        stream.write(&enc_msg).unwrap();
    }
}
