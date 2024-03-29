#[cfg(test)]
mod tests {
    use crate::syscalls::common::behaviour::Behaviour;
    use crate::syscalls::common::checkable::Checkable;
    use crate::syscalls::common::rule_result::RuleResult;
    use crate::syscalls::execv::condition::Condition;
    use crate::syscalls::execv::record::Record;
    use crate::syscalls::execv::rule::Rule;
    use crate::syscalls::execv::subject::Subject;

    #[test]
    fn check_by_pathname_succeeded() {
        let rule = Rule {
            subject: Subject::Pathname,
            condition: Condition::MustBeIn,
            objects: vec!["some/path".to_string()],
            behaviour_on_violation: Behaviour::KillProcess,
        };

        let record = Record {
            pathname: "some/path".to_string(),
            argv: None,
        };

        assert_eq!(RuleResult::Pass, rule.check(&record));
    }

    #[test]
    fn check_by_pathname_failed() {
        let rule = Rule {
            subject: Subject::Pathname,
            condition: Condition::MustBeIn,
            objects: vec!["some/path".to_string()],
            behaviour_on_violation: Behaviour::KillProcess,
        };

        let record = Record {
            pathname: "some/invalid/path".to_string(),
            argv: None,
        };

        assert_eq!(RuleResult::Fail, rule.check(&record));
    }

    #[test]
    fn check_by_argv_succeeded() {
        let rule = Rule {
            subject: Subject::Argv,
            condition: Condition::MustNotBeIn,
            objects: vec!["/etc/passwd".to_string()],
            behaviour_on_violation: Behaviour::KillProcess,
        };

        let record = Record {
            pathname: "/usr/bin/cat".to_string(),
            argv: Option::from(vec!["/var/log/sova.log".to_string()]),
        };

        assert_eq!(RuleResult::Pass, rule.check(&record));
    }

    #[test]
    fn check_by_argv_failed() {
        let rule = Rule {
            subject: Subject::Argv,
            condition: Condition::MustNotBeIn,
            objects: vec!["/etc/passwd".to_string()],
            behaviour_on_violation: Behaviour::KillProcess,
        };

        let record = Record {
            pathname: "/usr/bin/cat".to_string(),
            argv: Option::from(vec!["/etc/passwd".to_string()]),
        };

        assert_eq!(RuleResult::Fail, rule.check(&record));
    }
}
