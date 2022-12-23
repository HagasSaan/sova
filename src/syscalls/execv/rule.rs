use serde::{Deserialize, Serialize};

use crate::behaviour::Behaviour;
use crate::rule_result::RuleResult;
use crate::syscalls::execv::condition::Condition;
use crate::syscalls::execv::record::Record;
use crate::syscalls::execv::subject::Subject;


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
            Subject::Argv => self.check_by_argv(record),
        }
    }

    fn check_by_argv(&self, record: &Record) -> RuleResult {
        match &record.argv {
            None => { RuleResult::Pass },
            Some(argv) => {
                for arg in argv {
                    match self.condition {
                        Condition::MustBeIn => {
                            if !self.objects.contains(&arg) {
                                return RuleResult::Fail;
                            }
                        },
                        Condition::MustNotBeIn => {
                            if self.objects.contains(&arg) {
                                return RuleResult::Fail;
                            }
                        },
                    }
                }
                RuleResult::Pass
            },
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