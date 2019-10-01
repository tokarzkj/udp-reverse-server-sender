use std::net::UdpSocket;
use std::str;
use std::io;

fn main() {
    loop {
        println!("Enter your message");
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("couldn't read input");
        buffer = buffer.trim_end().to_string();
        if buffer == "quit" {
            break;
        }

        let socket = UdpSocket::bind("127.0.0.1:4242").expect("couldn't bind to address");
        socket
            .send_to(buffer.as_bytes(), "127.0.0.1:34254")
            .expect("couldn't send data");

        let mut buf = [0; 10];
        let (_amt, _src) = socket.recv_from(&mut buf).expect("couldn't receive from");
        println!("{}", str::from_utf8(&buf).unwrap());
    }
}
