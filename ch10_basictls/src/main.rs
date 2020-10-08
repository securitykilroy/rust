use native_tls::TlsConnector;
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let tlsconn = TlsConnector::new().unwrap();

    let stream = TcpStream::connect("www.google.com:443")
                        .expect("Unable to connect to the server");
    let mut tlsstream = tlsconn.connect("google.com", stream)
                        .expect("Can't create TLS connection");

    let request = String::from("GET / HTTP/1.1\r\nHost: www.google.com\r\n\r\n");

    let mut response = vec![];

    tlsstream.write_all(request.as_bytes())?;

    tlsstream.read_to_end(&mut response).unwrap();

    println!("{}", String::from_utf8_lossy(&response));

    Ok(())    
}