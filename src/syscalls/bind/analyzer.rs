use log::{debug, warn};

use crate::syscalls::bind::record::Record;
use crate::syscalls::common::behaviour::Behaviour;
use crate::syscalls::common::configuration::Configuration;
use crate::syscalls::common::rule_result::RuleResult;

pub struct Analyzer {
    configuration: Configuration,
}

impl Analyzer {
    pub fn new(configuration: Configuration) -> Self {
        Analyzer { configuration }
    }

    pub fn analyze(&self, record: Record) -> Behaviour {
        debug!("record: {:?}", &record);

        let rules = &self.configuration.rules.bind;

        if rules.is_none() {
            return Behaviour::LogOnly;
        }

        for rule in rules.as_ref().unwrap() {
            match rule.check(&record) {
                RuleResult::Pass => {}
                RuleResult::Fail => {
                    warn!("rule violated: {:?}", rule);
                    return rule.behaviour_on_violation.clone();
                }
            }
        }

        Behaviour::LogOnly
    }
}