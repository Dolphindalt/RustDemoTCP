use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;
use std::str;

const HOST: &str = "127.0.0.1:2727";
const MESSAGE_SIZE: usize = 256;

fn main() {
    // We take input for a username variable.
    println!("Please enter a username: ");
    let mut username = String::new();
    io::stdin().read_line(&mut username).ok();
    let username = username.trim();

    // Start up the tcp client.
    let mut client = TcpStream::connect(HOST).expect("Failed to create TCP client");
    client.set_nonblocking(true).expect("Failed to enable non-blocking option");

    // Create a channel for sending strings we wish to send.
    let (tx, rx) = mpsc::channel::<String>();

    // This thread will constantly try to read from the TCP server and tries to
    // receive a message to send to the TCP server.
    thread::spawn(move || loop {
        // The buffer must be a fixed size to simplify the client and server.
        let mut buff = vec![0; MESSAGE_SIZE];

        // We try to receive a message to send through the TCP server.
        match rx.try_recv() {
            Ok(msg) => {
                let mut msg = msg.into_bytes();
                msg.resize(MESSAGE_SIZE, 0);
                client.write_all(&msg).expect("Writing to server failed");
            },
            // Do nothing if empty.
            Err(TryRecvError::Empty) => (),
            // Handle this error so compiler is happy. We do nothing though,
            // the tx handle will error.
            Err(TryRecvError::Disconnected) => break
        }

        // This is demonstrated here as we listen for a message of the exact
        // size we specified.
        match client.read_exact(&mut buff) {
            Ok(_) => {
                let msg = buff.into_iter().collect::<Vec<_>>();
                let msg = str::from_utf8(&msg).expect("Failed to read utf8 string");
                println!("{}", msg);
            },
            // If the error due to blocking, then we block.
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("Lost connection to server");
                return;
            }
        }

        thread::sleep(Duration::from_millis(200));
    });

    println!("Welcome to the chat room. Please send a message: ");
    // The main thread will listen for user input and pass it to the channel,
    // which is receiving in the thread spawned earlier.
    loop {
        let mut buff = String::new();
        io::stdin().read_line(&mut buff).expect("Failed to read from stdin");
        let msg = buff.trim().to_string();
        if msg == ":q" {
            break
        }
        let msg = format!("{}: {}", &username, msg);
        if tx.send(msg).is_err() {
            break;
        }
    }
    println!("Terminating communication...");
}