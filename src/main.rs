use std::io::{Read, Write};
use std::net::*;
use abes_nice_things;
const PORT: u16 = 51235;
fn main() {
    if std::env::args().any(|arg| arg == "host") {
        host();
    } else if std::env::args().any(|arg| arg == "client") {
        client();
    } else {
        eprintln!("Give this either the argument host or client");
        loop {
            println!("Running as host or client?");
            let input = abes_nice_things::input().to_lowercase();
            match input {
                "host" => {host();}
                "client" => {client();}
                _ => {}
            }
        }
    }
}
fn host() {
    let stream = TcpListener::bind((Ipv4Addr::UNSPECIFIED, PORT))
        .unwrap()
        .accept()
        .unwrap()
        .0;
    printer(stream.try_clone().unwrap());
    sender(stream);
}
fn client() {
    println!("Who we connecting to?");
    let mut target = String::new();
    std::io::stdin().read_line(&mut target).unwrap();
    target = target.trim().to_string();
    let stream = TcpStream::connect(target).unwrap();
    printer(stream.try_clone().unwrap());
    sender(stream);
}
fn printer(mut stream: TcpStream) {
    std::thread::spawn(move || {
        let mut buf = [0];
        while let Ok(_) = stream.read_exact(&mut buf) {
            std::io::stdout().write_all(&[unscramble(buf[0])]).unwrap();
            std::io::stdout().flush().unwrap()
        }
    });
}
fn sender(mut stream: TcpStream) {
    loop {
        let mut buf = [0];
        std::io::stdin().read_exact(&mut buf).unwrap();
        if let Err(_) = stream.write_all(&[scramble(buf[0])]) {
            break;
        }
        stream.flush().unwrap()
    }
}
fn scramble(mut input: u8) -> u8 {
    input = input.rotate_left(5);
    input = input ^ 0b0101_1010;
    input = input.wrapping_add(57);
    input = input.rotate_right(3);
    input
}
fn unscramble(mut input: u8) -> u8 {
    input = input.rotate_left(3);
    input = input.wrapping_sub(57);
    input = input ^ 0b0101_1010;
    input = input.rotate_right(5);
    input
}
