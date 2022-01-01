mod record;
mod configuration;
mod analyzer;
mod sova;

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
    );

    let sova: Sova = Sova::new(configuration);

    sova.start().await
}
