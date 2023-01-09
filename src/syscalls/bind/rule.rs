use crate::syscalls::bind::condition::Condition;
use serde::{Deserialize, Serialize};

use crate::syscalls::bind::record::Record;
use crate::syscalls::bind::subject::Subject;
use crate::syscalls::common::behaviour::Behaviour;
use crate::syscalls::common::rule_result::RuleResult;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Rule {
    pub subject: Subject,
    pub condition: Condition,
    pub objects: Vec<String>,
    pub behaviour_on_violation: Behaviour,
}

impl Rule {
    pub fn check(&self, record: &Record) -> RuleResult {
        match self.subject {
            Subject::Port => self.check_by_port(record),
            Subject::Subnet => unimplemented!(), // TODO: implement
        }
    }

    fn check_by_port(&self, record: &Record) -> RuleResult {
        let port = &record.addr.sin_port.to_string();

        match self.condition {
            Condition::MustBeIn => {
                if !self.objects.contains(port) {
                    RuleResult::Fail
                } else {
                    RuleResult::Pass
                }
            }
            Condition::MustNotBeIn => {
                if self.objects.contains(port) {
                    RuleResult::Fail
                } else {
                    RuleResult::Pass
                }
            }
        }
    }
}
