#[cfg(test)]
mod tests {
    use crate::syscalls::common::behaviour::Behaviour;
    use crate::syscalls::common::rule_result::RuleResult;
    use crate::syscalls::execve::condition::Condition;
    use crate::syscalls::execve::record::Record;
    use crate::syscalls::execve::rule::Rule;
    use crate::syscalls::execve::subject::Subject;

    #[test]
    fn check_by_pathname_succeeded() {
        let rule = Rule {
            subject: Subject::Pathname,
            condition: Condition::MustBeIn,
            objects: vec![String::from("some/path")],
            behaviour_on_violation: Behaviour::KillProcess,
        };

        let record = Record {
            pathname: "some/path".to_string(),
            argv: None,
            envp: None,
        };

        assert_eq!(RuleResult::Pass, rule.check(&record));
    }

    #[test]
    fn check_by_pathname_failed() {
        let rule = Rule {
            subject: Subject::Pathname,
            condition: Condition::MustBeIn,
            objects: vec![String::from("some/path")],
            behaviour_on_violation: Behaviour::KillProcess,
        };

        let record = Record {
            pathname: "some/invalid/path".to_string(),
            argv: None,
            envp: None,
        };

        assert_eq!(RuleResult::Fail, rule.check(&record));
    }

    #[test]
    fn check_by_argv_succeeded() {
        let rule = Rule {
            subject: Subject::Argv,
            condition: Condition::MustNotBeIn,
            objects: vec![String::from("/etc/passwd")],
            behaviour_on_violation: Behaviour::KillProcess,
        };

        let record = Record {
            pathname: "/usr/bin/cat".to_string(),
            argv: Option::from(vec![String::from("/var/log/sova.log")]),
            envp: None,
        };

        assert_eq!(RuleResult::Pass, rule.check(&record));
    }

    #[test]
    fn check_by_argv_failed() {
        let rule = Rule {
            subject: Subject::Argv,
            condition: Condition::MustNotBeIn,
            objects: vec![String::from("/etc/passwd")],
            behaviour_on_violation: Behaviour::KillProcess,
        };

        let record = Record {
            pathname: "/usr/bin/cat".to_string(),
            argv: Option::from(vec![String::from("/etc/passwd")]),
            envp: None,
        };

        assert_eq!(RuleResult::Fail, rule.check(&record));
    }
}
