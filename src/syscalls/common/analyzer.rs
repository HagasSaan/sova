use crate::syscalls::common::behaviour::Behaviour;
use crate::syscalls::common::checkable::Checkable;
use crate::syscalls::common::rule_result::RuleResult;
use crate::syscalls::common::with_behaviour::WithBehaviour;
use log::{debug, warn};
use std::fmt::Debug;
use std::marker::PhantomData;

pub struct Analyzer<T: WithBehaviour + Checkable<R>, R> {
    pub rules: Option<Vec<T>>,
    record: PhantomData<R>,
}

impl<T: Debug + WithBehaviour + Checkable<R>, R: Debug> Analyzer<T, R> {
    pub fn new(rules: Option<Vec<T>>) -> Self {
        Analyzer {
            rules,
            record: PhantomData,
        }
    }

    pub fn analyze(&self, record: R) -> Behaviour {
        debug!("record: {:?}", &record);

        if self.rules.is_none() {
            return Behaviour::LogOnly;
        }

        for rule in self.rules.as_ref().unwrap() {
            match rule.check(&record) {
                RuleResult::Pass => {}
                RuleResult::Fail => {
                    warn!("rule violated: {:?}", rule);
                    return rule.behaviour();
                }
            }
        }

        Behaviour::LogOnly
    }
}
