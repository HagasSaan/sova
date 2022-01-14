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
}