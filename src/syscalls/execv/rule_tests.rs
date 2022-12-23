// TODO: move to tests dir

#[cfg(test)]
mod tests {
    use crate::behaviour::Behaviour;
    use crate::rule_result::RuleResult;
    use crate::syscalls::execv::condition::Condition;
    use crate::syscalls::execv::record::Record;
    use crate::syscalls::execv::rule::Rule;
    use crate::syscalls::execv::subject::Subject;

    #[test]
    fn check_by_pathname_succeeded() {
        let rule = Rule {
            subject: Subject::Pathname,
            condition: Condition::MustBeIn,
            objects: vec![String::from("some/path")],
            behaviour_on_violation: Behaviour::KillProcess
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
            objects: vec![String::from("some/path")],
            behaviour_on_violation: Behaviour::KillProcess
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
            objects: vec![String::from("/etc/passwd")],
            behaviour_on_violation: Behaviour::KillProcess
        };

        let record = Record {
            pathname: "/usr/bin/cat".to_string(),
            argv: Option::from(vec![String::from("/var/log/sova.log")]),
        };

        assert_eq!(RuleResult::Pass, rule.check(&record));
    }

    #[test]
    fn check_by_argv_failed() {
        let rule = Rule {
            subject: Subject::Argv,
            condition: Condition::MustNotBeIn,
            objects: vec![String::from("/etc/passwd")],
            behaviour_on_violation: Behaviour::KillProcess
        };

        let record = Record {
            pathname: "/usr/bin/cat".to_string(),
            argv: Option::from(vec![String::from("/etc/passwd")]),
        };

        assert_eq!(RuleResult::Fail, rule.check(&record));
    }

}