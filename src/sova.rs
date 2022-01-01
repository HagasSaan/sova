use tokio::net::UnixDatagram;
use std::{fs, io};

use crate::{Analyzer, Configuration, Record};
use crate::analyzer::AnalyzerResult;

pub struct Sova {
    configuration: Configuration,
    analyzer: Analyzer,
}

impl Sova {
    pub fn new(configuration: Configuration) -> Sova {
        let analyzer = Analyzer::new(configuration.clone());
        Sova {
            configuration,
            analyzer,
        }
    }

    pub async fn start(&self) -> io::Result<()>{
        match fs::remove_file(&self.configuration.unix_socket_path) {
            Ok(_) => { println!("Socket recreated") },
            Err(e) => {
                match e.kind() {
                    io::ErrorKind::NotFound => println!("Socket created"),
                    _ => panic!("Unexpected error on socket creation: {:?}", e),
                }
            },
        }

        let socket = UnixDatagram::bind(&self.configuration.unix_socket_path)?;

        println!("Listening started");

        loop {
            socket.readable().await?;
            let mut message: [u8; 1024] = [0; 1024];
            match socket.try_recv_from(&mut message) {
                Ok((message_size, _)) => {
                    match self.handle(message, message_size).await {
                        Ok(_) => continue,
                        Err(e) => { println!("Error while handling: {:?}", e); },
                    };
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

    async fn handle(&self, message: [u8; 1024], message_size: usize) -> Result<(), String> {
        let record = Record::from_bytes(&message, message_size)?;
        let analysis_result = self.analyzer.analyze(&record);
        println!("result: {:?}", record);
        println!("analysis result: {:?}", analysis_result);
        // TODO: realize actions
        match analysis_result {
            AnalyzerResult::LogOnly => {
                println!("Logging...");
            }
            AnalyzerResult::KillProcess => {
                println!("Killing process...");
            }
            AnalyzerResult::KillSystem => {
                println!("Death requested");
            }
        }

        Ok(())
    }
}
