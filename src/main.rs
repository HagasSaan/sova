use tokio::net::UnixDatagram;
use serde::{Serialize, Deserialize};
use std::{io, str};


#[derive(Serialize, Deserialize, Debug)]
struct Record {
    x: i32,
}

async fn handle(message: [u8; 1024]) -> io::Result<()>{
    let s = match str::from_utf8(&message) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    println!("result: {}", s);

    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let client_path = "/var/run/snoopy.sock";
    let socket = UnixDatagram::bind(&client_path)?;
    
    println!("Listening started\n");

    loop {
        socket.readable().await?;
        let mut message: [u8; 1024] = [0; 1024];
        match socket.try_recv_from(&mut message) {
            Ok((n, _addr)) => {
                handle(message).await?;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }
}
