#[cfg(test)]
mod tests {
    use crate::syscalls::common::behaviour::Behaviour;
    use crate::syscalls::common::checkable::Checkable;
    use crate::syscalls::common::rule_result::RuleResult;
    use crate::syscalls::open::condition::Condition;
    use crate::syscalls::open::record::Record;
    use crate::syscalls::open::rule::Rule;
    use crate::syscalls::open::subject::Subject;

    #[test]
    fn check_by_pathname_succeeded() {
        let rule = Rule {
            subject: Subject::Pathname,
            condition: Condition::MustBeIn,
            objects: vec!["/var/log/sova.log".to_string(), "/etc/passwd".to_string()],
            behaviour_on_violation: Behaviour::KillProcess,
        };

        let record = Record {
            pathname: "/etc/passwd".to_string(),
            flags: 0,
        };

        assert_eq!(RuleResult::Pass, rule.check(&record));
    }

    #[test]
    fn check_by_pathname_failed() {
        let rule = Rule {
            subject: Subject::Pathname,
            condition: Condition::MustBeIn,
            objects: vec!["/var/log/sova.log".to_string(), "/etc/passwd".to_string()],
            behaviour_on_violation: Behaviour::KillProcess,
        };

        let record = Record {
            pathname: "/bin/sh".to_string(),
            flags: 0,
        };

        assert_eq!(RuleResult::Fail, rule.check(&record));
    }
}
