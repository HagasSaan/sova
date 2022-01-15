use tokio::net::UnixDatagram;
use std::{fs, io, process};

use crate::{Analyzer, Behaviour, Configuration, Record};

// https://github.com/a2o/snoopy/blob/master/src/snoopy.h#L115
pub const SNOOPY_LOG_MESSAGE_MAX_SIZE: usize = 16383;

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
            let mut message: [u8; SNOOPY_LOG_MESSAGE_MAX_SIZE] = [0; SNOOPY_LOG_MESSAGE_MAX_SIZE];
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

    async fn handle(&self, message: [u8; SNOOPY_LOG_MESSAGE_MAX_SIZE], message_size: usize) -> Result<(), String> {
        let record = Record::from_bytes(&message, message_size)?;
        println!("record: {:?}", record);

        let behaviour = self.analyzer.analyze(&record);
        println!("Requested behaviour: {:?}", behaviour);

        self.handle_result(behaviour, record)
    }

    fn handle_result(&self, behaviour: Behaviour, record: Record) -> Result<(), String> {
        match behaviour {
            Behaviour::LogOnly => {
                println!("Logging...");
            }
            Behaviour::KillProcess => {
                println!(
                    "Killing process {} (pid: {}) due rule violation",
                    &record.cmdline,
                    &record.pid,
                );
                let mut process = process::Command::new("kill");
                process.arg("-9").arg(record.pid.clone());
                match process.status() {
                    Ok(_) => { println!("Process killed"); }
                    Err(_) => {
                        // TODO: handle it with configuration behaviour
                        println!("Process not killed");
                    }
                }
            }
            Behaviour::KillSystem => {
                println!("Killing system due rule violation");
                let mut process = process::Command::new("pkill");
                process.arg("-f").arg("/usr/bin/supervisor");
                match process.status() {
                    Ok(_) => { println!("System killed"); }
                    Err(_) => {
                        // TODO: ALARM!!!
                        println!("System not killed");
                    }
                }
            }
        }

        Ok(())
    }
}
