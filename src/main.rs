#![allow(unused)]
use std::net::TcpStream;
use std::thread;
fn main() {
    println!("Hello, world!");

    let addresses = [
        "192.168.31.1",
        "192.168.31.15",
        "192.168.31.23",
        "192.168.31.68",
        "192.168.31.107",
        "192.168.31.123",
        "192.168.31.178",
        "192.168.31.202",
    ];
    
    let handle = thread::spawn(move || {
        for ip in addresses {
        println!("{}",ip);

        can_be_reached(ip,80);

    }
    });


    handle.join().unwrap();

    // can_be_reached("192.168.1.100",80);
    
}

fn can_be_reached(ip: &str , port: i32) {
    if let Ok(stream) = TcpStream::connect(ip.to_string() +":"+&port.to_string()) {
    println!("the address has been reached");
} else {
    println!("Couldn't reached the address...");
}
}