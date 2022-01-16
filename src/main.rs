mod record;
mod configuration;
mod analyzer;
mod sova;
mod analyzer_tests;
mod record_tests;
mod rule;

use std::{env, io};
use std::env::VarError;
use record::Record;
use sova::Sova;
use crate::analyzer::Analyzer;
use crate::configuration::{Behaviour, Configuration};
use crate::rule::{ConditionType, Rule, Subject};

#[tokio::main]
async fn main() -> io::Result<()> {
    const DEFAULT_PATH: &str = "/tmp/sova.yaml";

    let path = match env::var("SOVA_CONFIG") {
        Ok(path) => path,
        Err(e) => {
            println!(
                "Config env var not found or corrupted due {:?}, using default path",
                e.to_string(),
            );
            String::from(DEFAULT_PATH)
        },
    };

    let configuration = Configuration::load(path);

    println!("Configuration loaded");

    let sova: Sova = Sova::new(configuration);

    sova.start().await
}
