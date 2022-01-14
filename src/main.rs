mod record;
mod configuration;
mod analyzer;
mod sova;
mod analyzer_tests;
mod record_tests;
mod rule;

use std::io;
use record::Record;
use sova::Sova;
use crate::analyzer::Analyzer;
use crate::configuration::{Behaviour, Configuration};
use crate::rule::{ConditionType, Rule, Subject};

#[tokio::main]
async fn main() -> io::Result<()> {
    let sock_path = "/var/run/snoopy.sock".to_string();
    let rules = vec![
        Rule {
            subject: Subject::CommandLine,
            condition: ConditionType::MustBeIn,
            objects: vec![String::from("telegram-desktop")],
            behaviour_on_violation: Behaviour::KillProcess
        }
    ];

    let configuration = Configuration::new(
        sock_path,
        Behaviour::KillProcess,
        rules,
    );

    let sova: Sova = Sova::new(configuration);

    sova.start().await
}
