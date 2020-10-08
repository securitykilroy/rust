
use native_tls::{Identity, TlsAcceptor, TlsStream};
use std::fs::File;
use std::io::{Read};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

fn main() {
    let mut cert = File::open("certificate.pfx").unwrap();
    let mut myidentity = vec![];
    cert.read_to_end(&mut myidentity).unwrap();
    let certidentity = Identity::from_pkcs12(&myidentity, "passphrase").unwrap();

    let tcplistener = TcpListener::bind("0.0.0.0:7999").unwrap();
    let acceptor = TlsAcceptor::new(certidentity).unwrap();
    let acceptor = Arc::new(acceptor);

    for clientstream in tcplistener.incoming() {
        match clientstream {
            Ok(clientstream) => {
                let acceptor = acceptor.clone();
                thread::spawn(move || {
                    let clientstream = acceptor.accept(clientstream).unwrap();
                   /* do something here using clientstream */
                });
            }
            Err(e) => { /* connection failed */ }
        }
    }
}
