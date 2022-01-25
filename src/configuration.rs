use std::fs::File;
use std::io::{Read, Write};
use serde::{Deserialize, Serialize};

use crate::behaviour::Behaviour;
use crate::rule::Rule;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Configuration {
    pub behaviour_on_incidents: Behaviour,
    pub rules: Vec<Rule>,
    pub logfile_path: String,
}

impl Configuration {
    pub fn load(path: &str) -> Result<Self, &str> {
        let mut input = File::open(path)
            .expect("Error on opening file");

        let mut config_str = String::new();
        input.read_to_string(&mut config_str)
            .expect("Error on reading file");

        let configuration = serde_yaml::from_str(&*config_str)
            .expect("Error on loading configuration from content");

        Ok(configuration)
    }

    pub fn save(&self, path: &str) -> Result<(), &str> {
        let config_str = serde_yaml::to_string(&self)
            .expect("Error on dumping config to string");

        let mut output = File::create(path)
            .expect("Error on creating file");

        output.write_all(config_str.as_ref())
            .expect("Error on writing content to file");

        Ok(())
    }
}