use serde::{Deserialize, Serialize};
use crate::configuration::Behaviour;
use crate::Record;

#[derive(Clone, Deserialize, Serialize, Hash, Eq, PartialEq)]
pub enum Subject {
    CommandLine,
    UserID,  // TODO: in progress
    Filename, // TODO: in progress
    // NOTE: subjects of record // TODO: move to documentation
}

#[derive(Clone, Deserialize, Serialize, Hash, Eq, PartialEq)]
pub enum ConditionType {
    In,
    NotIn,
}

#[derive(Clone, Deserialize, Serialize, Hash, Eq, PartialEq)]
pub enum RuleResult {
    Pass,
    Fail,
}

#[derive(Clone, Deserialize, Serialize, Hash, Eq, PartialEq)]
pub struct Rule {
    pub subject: Subject,
    pub condition: ConditionType,
    pub objects: Vec<String>,
    pub behaviour_on_violation: Behaviour,
}

impl Rule {
    pub fn check(&self, record: &Record) -> Result<RuleResult, String> {
        let subject = self.get_subject(record)?;
        match self.condition {
            ConditionType::In => {
                if self.objects.contains(&subject) {
                    Ok(RuleResult::Pass)
                } else {
                    Ok(RuleResult::Fail)
                }
            },
            ConditionType::NotIn => {
                if self.objects.contains(&subject) {
                    Ok(RuleResult::Fail)
                } else {
                    Ok(RuleResult::Pass)
                }
            }
        }
    }

    fn get_subject(&self, record: &Record) -> Result<String, String> {
        match self.subject {
            Subject::CommandLine => Ok(record.cmdline.clone()),
            Subject::Filename => Ok(record.filename.clone()),
            _ => Err(String::from("Unknown subject")),
        }
    }
}
