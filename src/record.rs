use serde::{Deserialize, Serialize};
use std::str;

#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    cmdline: String,
    cwd: String,
    datetime: String,
    domain: String,
    hostname: String,
    egid: String,
    euid: String,
    egroup: String,
    login: String,
    pid: String,
    ppid: String,
    tty: String,
    tty_uid: String,
    tty_username: String,
    username: String,
    eusername: String,
    uid: String,
    ssh: String,
    filename: String,
    gid: String,
    group: String,
    rpname: String,
    sid: String,
    tid: String,
    tid_kernel: String,
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