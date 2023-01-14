use crate::syscalls::connect::condition::Condition;
use serde::{Deserialize, Serialize};
use crate::syscalls::common::with_behaviour::WithBehaviour;

use crate::syscalls::common::behaviour::Behaviour;
use crate::syscalls::common::checkable::Checkable;
use crate::syscalls::common::rule_result::RuleResult;
use crate::syscalls::connect::record::Record;
use crate::syscalls::connect::subject::Subject;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Rule {
    pub subject: Subject,
    pub condition: Condition,
    pub objects: Vec<String>,
    pub behaviour_on_violation: Behaviour,
}

impl Rule {
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

impl WithBehaviour for Rule {
    fn behaviour(&self) -> Behaviour {
        self.behaviour_on_violation.clone()
    }
}

impl Checkable<Record> for Rule {
    fn check(&self, record: &Record) -> RuleResult {
        match self.subject {
            Subject::Port => self.check_by_port(record),
            Subject::Subnet => unimplemented!(), // TODO: implement
        }
    }
}