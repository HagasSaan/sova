use crate::syscalls::common::rule_result::RuleResult;

pub trait Checkable<R> {
    fn check(&self, record: &R) -> RuleResult;
}
