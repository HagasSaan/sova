#[cfg(test)]
mod tests {
    use crate::behaviour::Behaviour;
    use crate::condition::Condition;
    use crate::record::Record;
    use crate::rule::Rule;
    use crate::rule_result::RuleResult;
    use crate::subject::Subject;

    #[test]
    fn check_by_path_succeeded() {
        let rule = Rule {
            subject: Subject::Path,
            condition: Condition::MustBeIn,
            objects: vec![String::from("some/path")],
            behaviour_on_violation: Behaviour::KillProcess
        };

        let record = Record {
            path: "some/path".to_string(),
            argv: None,
            envp: None
        };

        assert_eq!(RuleResult::Pass, rule.check(&record));
    }

    #[test]
    fn check_by_path_failed() {
        let rule = Rule {
            subject: Subject::Path,
            condition: Condition::MustBeIn,
            objects: vec![String::from("some/path")],
            behaviour_on_violation: Behaviour::KillProcess
        };

        let record = Record {
            path: "some/invalid/path".to_string(),
            argv: None,
            envp: None
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
            path: "/usr/bin/cat".to_string(),
            argv: Option::from(vec![String::from("/var/log/sova.log")]),
            envp: None
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
            path: "/usr/bin/cat".to_string(),
            argv: Option::from(vec![String::from("/etc/passwd")]),
            envp: None
        };

        assert_eq!(RuleResult::Fail, rule.check(&record));
    }

}