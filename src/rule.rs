use std::collections::HashSet;
use serde::{Deserialize, Serialize};
use std::io;
use crate::Record;

#[derive(Clone, Deserialize, Serialize, Hash, Eq, PartialEq)]
enum Subject {
    Process,
    UserID,
    Filename,
    // NOTE: subjects of record // TODO: move to documentation
}

#[derive(Clone, Deserialize, Serialize, Hash, Eq, PartialEq)]
enum ConditionType {
    In,
    NotIn,
}

#[derive(Clone, Deserialize, Serialize, Hash, Eq, PartialEq)]
enum RuleResult {
    Pass,
    Fail,
}

#[derive(Clone, Deserialize, Serialize, Hash, Eq, PartialEq)]
pub struct Rule {
    subject: Subject,
    condition: ConditionType,
    objects: HashSet<String>,
}

impl Rule {
    pub fn check(&self, record: Record) -> io::Result<RuleResult> {
        match self.condition {
            ConditionType::In => {
                // get record subject
                // if subject in objects return Ok(RuleResult::Pass)
                // otherwise return Ok(RuleResult::Fail)
            },
            ConditionType::NotIn => {
                // get record subject
                // if subject not in objects return Ok(RuleResult::Pass)
                // otherwise return Ok(RuleResult::Fail)
            }
        }
        Ok(RuleResult::Pass)
    }
}
