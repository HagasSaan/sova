use std::collections::HashSet;
use serde::{Deserialize, Serialize};
use std::io;
use crate::Record;

enum Subject {
    Process,
    UserID,
    Filename,
    // NOTE: subjects of record
}

enum ConditionType {
    In,
    NotIn,
}

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
        match condition {
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
