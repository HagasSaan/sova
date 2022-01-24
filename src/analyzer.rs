use Record;
use rule::RuleResult;
use crate::behaviour::Behaviour;
use crate::configuration::Configuration;
use log::{debug, error, info, trace, warn};

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

        for rule in &self.configuration.rules {
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