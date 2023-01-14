use log::{debug, warn};

use crate::syscalls::bind::record::Record;
use crate::syscalls::bind::rule::Rule;
use crate::syscalls::common::analyzer::Analyzer;
use crate::syscalls::common::behaviour::Behaviour;
use crate::syscalls::common::rule_result::RuleResult;

impl Analyzer<Rule> {
    pub fn new(rules: Option<Vec<Rule>>) -> Self {
        Analyzer { rules }
    }

    pub fn analyze(&self, record: Record) -> Behaviour {
        debug!("record: {:?}", &record);

        if self.rules.is_none() {
            return Behaviour::LogOnly;
        }

        for rule in self.rules.as_ref().unwrap() {
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
