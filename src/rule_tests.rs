#[cfg(test)]
mod tests {
    use crate::{Behaviour, Record};
    use crate::condition::Condition;
    use crate::rule::Rule;
    use crate::rule_result::RuleResult;
    use crate::subject::Subject;

    fn get_rule() -> Rule {
        Rule {
            subject: Subject::Path,
            condition: Condition::MustBeIn,
            objects: vec![String::from("some/path")],
            behaviour_on_violation: Behaviour::KillProcess
        }
    }

    #[test]
    fn check_by_path_succeeded() {
        let rule = get_rule();

        let valid_record = Record {
            path: "some/path".to_string(),
            argv: None,
            envp: None
        };

        assert_eq!(RuleResult::Pass, rule.check(&valid_record));
    }

    #[test]
    fn check_by_path_failed() {
        let rule = get_rule();

        let invalid_record = Record {
            path: "some/invalid/path".to_string(),
            argv: None,
            envp: None
        };

        assert_eq!(RuleResult::Fail, rule.check(&invalid_record));
    }

}