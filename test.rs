use std::net::UdpSocket;

fn main() {
    let sock = UdpSocket::bind("0.0.0.0:1150").unwrap();
    println!("Connected! {}", sock.local_addr().unwrap());
    loop {}
}
