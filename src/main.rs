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
use crate::configuration::{BehaviourOnIncidents, Configuration};

#[tokio::main]
async fn main() -> io::Result<()> {
    let sock_path = "/var/run/snoopy.sock".to_string();

    let configuration = Configuration::new(
        sock_path,
        BehaviourOnIncidents::LogOnly,
        None
    );

    let sova: Sova = Sova::new(configuration);

    sova.start().await
}
