#[macro_use]
extern crate lazy_static;
extern crate libc;
extern crate log;

mod utils;
mod analyzer;
mod behaviour;
mod rule;
mod record;
mod configuration;
mod logger;
mod syscalls;
mod condition;
mod subject;
mod rule_result;

mod rule_tests;
