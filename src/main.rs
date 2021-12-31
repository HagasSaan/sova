use tokio::net::UnixDatagram;
use std::{fs, io, str};
use record::Record;

mod record;

async fn handle(message: [u8; 1024], message_size: usize) -> io::Result<()>{
    let s: Record = match str::from_utf8(&message[..message_size]) {
        Ok(v) => {
            match serde_json::from_str(v) {
                Ok(v) => v,
                Err(e) => panic!("String is not valid JSON: {}", e),
            }
        },
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    println!("result: {:?}", s);

    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let sock_path = "/var/run/snoopy.sock";

    match fs::remove_file(sock_path) {
        Ok(_) => {println!("Socket recreated")}
        Err(e) => { println!("Error: {:?}", e)}
    }

    let socket = UnixDatagram::bind(&sock_path)?;
    
    println!("Listening started");

    loop {
        socket.readable().await?;
        let mut message: [u8; 1024] = [0; 1024];
        match socket.try_recv_from(&mut message) {
            Ok((message_size, _)) => {
                handle(message, message_size).await?;
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
