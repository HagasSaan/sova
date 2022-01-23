use behaviour::Behaviour;
use Record;

#[derive(Debug)]
pub enum ConditionType {
    MustBeIn,
    MustNotBeIn,
}

pub enum RuleResult {
    Pass,
    Fail,
}

#[derive(Debug)]
pub struct Rule {
    pub subject: String,
    pub condition: ConditionType,
    pub objects: Vec<String>,
    pub behaviour_on_violation: Behaviour,
}

impl Rule {
    pub fn check(&self, record: &Record) -> RuleResult {
        // TODO: check subject
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
}