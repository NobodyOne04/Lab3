use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;

use super::protector::*;

const MESSAGE_SIZE: usize = 32;
const SLEEP_TIME: u64 = 100;

pub fn run(localhost:String) {
    let server = TcpListener::bind(localhost).expect("Listener failed to bind");
    server.set_nonblocking(true).expect("Failed to initialize nonblocking");
    let mut clients = Vec::new();
    let (sx, rx) = mpsc::channel::<String>();
    loop {
        if let Ok((mut socket, addr)) = server.accept() {
            let mut access_key = "".to_string();
            let mut buf_hash = vec![0; MESSAGE_SIZE];
            println!("Client {} connected", addr);
            let sx = sx.clone();
            socket.read_exact(&mut buf_hash);
            let hash = buf_hash.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
            let hash = String::from_utf8(hash).expect("Invalid utf8 message");
            println!("hash : {:?}", hash);
            let protect = super::protector::SessionProtector{hash: hash};
            clients.push(socket.try_clone().expect("Failed to clone client"));
            thread::spawn(move || loop {
                let (mut buf, mut key_buf) = (vec![0; MESSAGE_SIZE], vec![0; MESSAGE_SIZE]);
                match socket.read_exact(&mut key_buf) {
                    Ok(_) => {
                        let key = key_buf.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                        let key = String::from_utf8(key).expect("Invalid utf8 message");
                        println!("key : {:?}", key);
                        access_key = if access_key == "" {key.clone()} else {protect.next_session_key(&access_key.clone())};
                        if access_key != key {println!("server : {}, client : {}", access_key,  key);}
                    },
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                    Err(_) => {break;}
                }
                match socket.read_exact(&mut buf) {
                    Ok(_) => {
                        let msg = buf.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                        let msg = String::from_utf8(msg).expect("Invalid utf8 message");
                        sx.send(msg).expect("Send to master channel failed");
                    },
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                    Err(_) => {break;}
                }
                thread::sleep(::std::time::Duration::from_millis(SLEEP_TIME));
            });
        }
        if let Ok(msg) = rx.try_recv() {
            clients = clients.into_iter().filter_map(|mut client| {
                let mut buf = msg.clone().into_bytes();
                buf.resize(MESSAGE_SIZE, 0);
                client.write_all(&buf).map(|_| client).ok()
            }).collect::<Vec<_>>();
        }
        thread::sleep(::std::time::Duration::from_millis(SLEEP_TIME));
    }
}