use rsa::pkcs8::EncodePublicKey;
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use std::io::{self, Read, Write};
use std::net::TcpListener;
use std::thread;

fn main() {
    let mut rng = rand::thread_rng();
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);

    let listener = TcpListener::bind("127.0.0.1:9123").unwrap();
    println!("listening started, ready to accept");
    for stream in listener.incoming() {
        let pub_key_clone = pub_key.clone();
        let priv_key_clone = priv_key.clone();

        thread::spawn(move || {
            let mut stream = stream.unwrap();
            println!("accepted conn from : {}", stream.peer_addr().unwrap());
            let msg = pub_key_clone.to_public_key_der().unwrap();

            stream.write(msg.as_bytes()).unwrap();
            let mut buf = [0; 1024];

            loop {
                let bytes_read = stream.read(&mut buf).unwrap();
                if bytes_read == 0 {
                    break;
                }

                println!("encryped bytes : {:?}", &buf[..bytes_read]);

                let text = priv_key_clone
                    .decrypt(Pkcs1v15Encrypt, &buf[..bytes_read])
                    .unwrap();

                println!("decryped text : {}", String::from_utf8_lossy(&text));
                io::stdout().flush().unwrap();

                buf = [0; 1024];
            }
        });
    }
}
