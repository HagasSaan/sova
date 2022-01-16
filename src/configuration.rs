use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error, Read};
use serde::{Deserialize, Serialize};

use crate::rule::Rule;

#[derive(Clone, Deserialize, Serialize, PartialEq, Hash, Debug, Eq)]
pub enum Behaviour {
    LogOnly,
    KillProcess,
    KillSystem,
}

#[derive(Clone, Deserialize, Serialize, PartialEq)]
pub struct Configuration {
    pub unix_socket_path: String,
    pub behaviour_on_incidents: Behaviour,
    pub rules: Vec<Rule>,
}

impl Configuration {
    pub fn new(
        unix_socket_path: String,
        behaviour_on_incidents: Behaviour,
        rules: Vec<Rule>,
    ) -> Configuration {
        Configuration {
            unix_socket_path,
            behaviour_on_incidents,
            rules
        }
    }

    pub fn dump_into_yaml_file(&self, path: String) -> () {
        let config_str = serde_yaml::to_string(&self)
            .expect("Error on dumping config to string");

        let mut output = File::create(path).expect("Error on creating file");

        output.write_all(config_str.as_ref()).expect("Error on writing content to file");

        ()
    }

    pub fn load_from_yaml_file(path: String) -> Self {
        let mut input = File::open(path).expect("Error on opening file");
        let mut config_str = String::new();
        input.read_to_string(&mut config_str).expect("Error on reading file");

        let configuration = serde_yaml::from_str(&*config_str)
            .expect("Error on loading configuration from content");

        configuration
    }
}