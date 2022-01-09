use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::rule::Rule;

#[derive(Clone, Deserialize, Serialize, PartialEq)]
pub enum BehaviourOnIncidents {
    LogOnly,
    KillProcess,
    KillSystem,
}

#[derive(Clone, Deserialize, Serialize, PartialEq)]
pub struct Configuration {
    pub unix_socket_path: String,
    pub behaviour_on_incidents: BehaviourOnIncidents,
    pub rules: Option<HashSet<Rule>>,
}

impl Configuration {
    pub fn new(
        unix_socket_path: String,
        behaviour_on_incidents: BehaviourOnIncidents,
        rules: Option<HashSet<Rule>>,
    ) -> Configuration {
        Configuration {
            unix_socket_path,
            behaviour_on_incidents,
            rules
        }
    }
}