use serde::{Deserialize, Serialize};
use std::str;

#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    pub cmdline: String,
    pub cwd: String,
    pub datetime: String,
    pub domain: String,
    pub hostname: String,
    pub egid: String,
    pub euid: String,
    pub egroup: String,
    pub login: String,
    pub pid: String,
    pub ppid: String,
    pub tty: String,
    pub tty_uid: String,
    pub tty_username: String,
    pub username: String,
    pub eusername: String,
    pub uid: String,
    pub ssh: String,
    pub filename: String,
    pub gid: String,
    pub group: String,
    pub rpname: String,
    pub sid: String,
    pub tid: String,
    pub tid_kernel: String,
}

impl Record {
    pub fn from_bytes(message: &[u8; 1024], message_size: usize) -> Result<Record, String> {
        match str::from_utf8(&message[..message_size]) {
            Ok(v) => {
                match serde_json::from_str(v) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(e.to_string()),
                }
            },
            Err(e) => Err(e.to_string()),
        }
    }
}