#[cfg(test)]
mod analyzer_tests {
    use crate::{Analyzer, BehaviourOnIncidents, Configuration, Record};

    fn analyzer_log_only() -> Analyzer {
        let configuration: Configuration = Configuration::new(
            "/tmp/test.sock".to_string(),
            BehaviourOnIncidents::LogOnly,
        );
        Analyzer::new(configuration)
    }

    fn record() -> Record {
        unimplemented!()
    }

    #[test]
    fn test_analyzer_creation() {
        analyzer_log_only();
    }

    #[test]
    fn test_analyzer_logs_string() {
        unimplemented!();
    }

    #[test]
    fn test_analyzer_requests_process_kill_on_not_whitelisted_log() {
        unimplemented!();
    }

    #[test]
    fn test_analyzer_requests_system_kill_on_not_whitelisted_log() {
        unimplemented!();
    }
}
