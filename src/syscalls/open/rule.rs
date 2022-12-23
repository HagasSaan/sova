use serde::{Deserialize, Serialize};

use crate::behaviour::Behaviour;
use crate::rule_result::RuleResult;
use crate::syscalls::open::condition::Condition;
use crate::syscalls::open::subject::Subject;
use crate::syscalls::open::record::Record;

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
            Subject::Pathname => self.check_by_pathname(record),
            Subject::PathnameAndFlags => unimplemented!(),
        }
    }

    fn check_by_pathname(&self, record: &Record) -> RuleResult {
        match self.condition {
            Condition::MustBeIn => {
                if !self.objects.contains(&record.pathname) {
                    RuleResult::Fail
                } else {
                    RuleResult::Pass
                }
            },
            Condition::MustNotBeIn => {
                if self.objects.contains(&record.pathname) {
                    RuleResult::Fail
                } else {
                    RuleResult::Pass
                }
            },
        }
    }
}