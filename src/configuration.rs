use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use serde::{Deserialize, Serialize};

use crate::behaviour::Behaviour;
use crate::syscalls::execv::rule::Rule as ExecvRule;
use crate::syscalls::execve::rule::Rule as ExecveRule;
use crate::syscalls::open::rule::Rule as OpenRule;


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Rules {
    pub execv: Option<Vec<ExecvRule>>,
    pub execve: Option<Vec<ExecveRule>>,
    pub open: Option<Vec<OpenRule>>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Configuration {
    pub behaviour_on_incidents: Behaviour,
    pub logfile_path: Option<String>,
    pub rules: Rules
}

impl Configuration {
    pub fn load(path: &str) -> Configuration {
        let mut input = File::open(path)
            .expect("Error on opening file");

        let mut config_str = String::new();
        input.read_to_string(&mut config_str)
            .expect("Error on reading file");

        serde_yaml::from_str(&config_str)
            .expect("Error on loading configuration from content")
    }

    pub fn save(&self, path: &str) {
        let config_str = serde_yaml::to_string(&self)
            .expect("Error on dumping config to string");

        let mut output = File::create(path)
            .expect("Error on creating file");

        output.write_all(config_str.as_ref())
            .expect("Error on writing content to file");
    }
}

fn build_default_configuration() -> Configuration {
    Configuration {
        behaviour_on_incidents: Behaviour::LogOnly,
        logfile_path: None,
        rules: Rules {
            execv: None,
            execve: None,
            open: None
        }
    }
}

pub fn load_configuration() -> Configuration {
    let config_path = "/etc/sova/sova.yaml";

    if !Path::new(config_path).exists() {
        fs::create_dir_all("/etc/sova")
            .expect("Failed to create /etc/sova directory");
        build_default_configuration()
            .save(config_path);
    }


    Configuration::load(config_path)
}
