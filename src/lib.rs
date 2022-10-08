#[macro_use]
extern crate lazy_static;
extern crate libc;
extern crate log;

use analyzer::Analyzer;
use behaviour::Behaviour;
use logger::setup_logger;
use record::Record;

mod utils;
mod analyzer;
mod behaviour;
mod rule;
mod record;
mod configuration;
mod logger;
mod syscalls;
