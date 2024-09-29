use core::str;
use std::{env, error::Error};
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let address = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8125".to_string());
    let mut buffer = [0; 1024];

    let socket = UdpSocket::bind(&address).await?;
    println!("Listening on: {}", socket.local_addr()?);

    loop {
        let (len, addr) = socket.recv_from(&mut buffer).await?;
        println!("{:?} bytes received from {:?}", len, addr);

        if let Ok(message) = str::from_utf8(&buffer[..len]) {
            println!("Received message: {}", message);
        } else {
            println!("Non-UTF8 message received, not valid");
        }
    }
}
