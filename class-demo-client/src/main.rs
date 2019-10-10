use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;
use std::str;

const HOST: &str = "127.0.0.1:2727";
const MESSAGE_SIZE: usize = 256;

fn main() {
    
}