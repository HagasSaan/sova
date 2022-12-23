use log::{debug, warn};

use crate::behaviour::Behaviour;
use crate::configuration::Configuration;
use crate::rule_result::RuleResult;
use crate::syscalls::execve::record::Record;

pub struct Analyzer {
    configuration: Configuration,
}

impl Analyzer {
    pub fn new(configuration: Configuration) -> Self {
        Analyzer {
            configuration
        }
    }

    pub fn analyze(&self, record: Record) -> Behaviour {
        debug!("record: {:?}", &record);

        let rules = &self.configuration.rules.execve;

        if let None = rules {
            return Behaviour::LogOnly;
        }

        for rule in rules.as_ref().unwrap() {
            match rule.check(&record) {
                RuleResult::Pass => {},
                RuleResult::Fail => {
                    warn!("rule violated: {:?}", rule);
                    return rule.behaviour_on_violation.clone()
                },
            }
        }

        Behaviour::LogOnly
    }
}
