use crate::configuration::Configuration;
use crate::{BehaviourOnIncidents, Record};

#[derive(Debug)]
pub enum AnalyzerResult {
    LogOnly,
    KillProcess,
    KillSystem,
}

pub struct Analyzer {
    configuration: Configuration,
}

impl Analyzer {
    pub fn new(configuration: Configuration) -> Analyzer { Analyzer { configuration } }

    pub fn analyze(&self, record: &Record) -> AnalyzerResult {
        match self.configuration.behaviour_on_incidents {
            BehaviourOnIncidents::LogOnly => AnalyzerResult::LogOnly,
            _ => {
                // TODO: if process not in whitelist or something like that
                AnalyzerResult::LogOnly
            }
        }
    }
}