use serde::{Deserialize, Serialize};

use crate::behaviour::Behaviour;
use crate::Record;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Subject {
    Path,
    PathArgs,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ConditionType {
    MustBeIn,
    MustNotBeIn,
}

pub enum RuleResult {
    Pass,
    Fail,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Rule {
    pub subject: Subject,
    pub condition: ConditionType,
    pub objects: Vec<String>,
    pub behaviour_on_violation: Behaviour,
}

impl Rule {
    pub fn check(&self, record: &Record) -> RuleResult {
        match self.subject {
            Subject::Path => {
                match self.condition {
                    ConditionType::MustBeIn => {
                        if !self.objects.contains(&record.path) {
                            RuleResult::Fail
                        } else {
                            RuleResult::Pass
                        }
                    },
                    ConditionType::MustNotBeIn => {
                        if self.objects.contains(&record.path) {
                            RuleResult::Fail
                        } else {
                            RuleResult::Pass
                        }
                    },
                }
            }
            _ => {unimplemented!()}
        }

    }
}