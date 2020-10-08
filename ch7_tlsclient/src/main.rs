use native_tls::TlsConnector;
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    let tlsconn = TlsConnector::builder().unwrap().build().unwrap();

    let tcpstream = TcpStream::connect("www.wrox.com:443").unwrap();
    let mut tlsstream = tlsconn.connect("www.wrox.com", tcpstream).unwrap();

    tlsstream.write_all(b"GET / HTTP/1.1\nHost: www.wrox.com\n\n").unwrap();
    let mut response = vec![];
    tlsstream.read_to_end(&mut response).unwrap();
    println!("{}", String::from_utf8_lossy(&response));

}
