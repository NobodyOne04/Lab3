use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;

const MESSAGE_SIZE: usize = 32;
const SLEEP_TIME: u64 = 100;

pub fn run(localhost:String) {
    let server = TcpListener::bind(localhost).expect("Listener failed to bind");
    server.set_nonblocking(true).expect("Failed to initialize nonblocking");
    let mut clients = Vec::new();
    let (sx, rx) = mpsc::channel::<String>();
    loop {
        if let Ok((mut socket, addr)) = server.accept() {
            println!("Client {} connected", addr);
            let sx = sx.clone();
            clients.push(socket.try_clone().expect("Failed to clone client"));
            thread::spawn(move || loop {
                let mut buf = vec![0; MESSAGE_SIZE];
                match socket.read_exact(&mut buf) {
                    Ok(_) => {
                        let msg = buf.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                        let msg = String::from_utf8(msg).expect("Invalid utf8 message");
                        println!("{}: {:?}", addr, msg);
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