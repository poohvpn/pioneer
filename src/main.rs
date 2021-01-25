use std::net::UdpSocket;

fn main() {
    let sock = UdpSocket::bind("0.0.0.0:11111").unwrap();
    let buf =&mut [0;1<<16];
    loop {
        let (n,addr) = sock.recv_from(buf).unwrap();
        sock.send_to(&buf[..n],addr).unwrap();
    }
}
