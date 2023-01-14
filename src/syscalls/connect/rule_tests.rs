#[cfg(test)]
mod tests {
    use crate::syscalls::common::behaviour::Behaviour;
    use crate::syscalls::common::checkable::Checkable;
    use crate::syscalls::common::rule_result::RuleResult;
    use crate::syscalls::common::sockaddr_in::SockaddrIn;
    use crate::syscalls::connect::condition::Condition;
    use crate::syscalls::connect::record::Record;
    use crate::syscalls::connect::rule::Rule;
    use crate::syscalls::connect::subject::Subject;
    use std::net::Ipv4Addr;

    #[test]
    fn check_by_port_succeeded() {
        let rule = Rule {
            subject: Subject::Port,
            condition: Condition::MustBeIn,
            objects: vec!["111".to_string(), "222".to_string()],
            behaviour_on_violation: Behaviour::KillProcess,
        };

        let record = Record {
            sockfd: 0,
            addr: SockaddrIn {
                sin_family: 2,
                sin_port: 111,
                sin_addr: Ipv4Addr::new(0, 0, 0, 0),
                sin_zero: [0, 0, 0, 0, 0, 0, 0, 0],
            },
            addrlen: 0,
        };

        assert_eq!(RuleResult::Pass, rule.check(&record));
    }

    #[test]
    fn check_by_port_failed() {
        let rule = Rule {
            subject: Subject::Port,
            condition: Condition::MustBeIn,
            objects: vec!["111".to_string(), "222".to_string()],
            behaviour_on_violation: Behaviour::KillProcess,
        };

        let record = Record {
            sockfd: 0,
            addr: SockaddrIn {
                sin_family: 2,
                sin_port: 123,
                sin_addr: Ipv4Addr::new(0, 0, 0, 0),
                sin_zero: [0, 0, 0, 0, 0, 0, 0, 0],
            },
            addrlen: 0,
        };

        assert_eq!(RuleResult::Fail, rule.check(&record));
    }
}
