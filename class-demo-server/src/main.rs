use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;
use std::io::{ErrorKind, Read, Write};

const HOST: &str = "127.0.0.1:2727";
const MAX_MESSAGE_SIZE: usize = 256;

fn main() {
    let server = TcpListener::bind(HOST).expect("Failed to start TCP listener");
    server.set_nonblocking(true).expect("Failed to set non-blocking option");

    let mut clients = vec![];
    let (tx, rx) = mpsc::channel::<String>();
    loop {
        if let Ok((mut socket, addr)) = server.accept() {
            println!("Client {} connected", addr);

            let tx = tx.clone();
            clients.push(socket.try_clone().expect("Failed to clone client"));

            thread::spawn(move || loop {
                let mut buff = vec![0; MAX_MESSAGE_SIZE];

                match socket.read_exact(&mut buff) {
                    Ok(_) => {
                        let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                        let msg = String::from_utf8(msg).expect("The message was not a valid utf-8 string");
                        println!("{}: {}", addr, msg);
                        tx.send(msg).expect("Failed to send message");
                    },
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                    Err(_) => {
                        println!("Closing {}\'s connection", addr);
                        break;
                    }
                }

                thread::sleep(std::time::Duration::from_millis(200));
            });
        }

        if let Ok(msg) = rx.try_recv() {
            clients = clients.into_iter().filter_map(|mut client| {
                let mut buff = msg.clone().into_bytes();
                buff.resize(MAX_MESSAGE_SIZE, 0);
                client.write_all(&buff).map(|_| client).ok()
            }).collect::<Vec<_>>();
        }

        thread::sleep(std::time::Duration::from_millis(200));
    }
}