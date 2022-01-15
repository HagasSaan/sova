use crate::configuration::{Behaviour, Configuration};
use crate::Record;
use crate::rule::RuleResult;

pub struct Analyzer {
    configuration: Configuration,
}

impl Analyzer {
    pub fn new(configuration: Configuration) -> Analyzer { Analyzer { configuration } }

    pub fn analyze(&self, record: &Record) -> Behaviour {
        match self.configuration.behaviour_on_incidents {
            Behaviour::LogOnly => Behaviour::LogOnly,
            _ => {
                for rule in &self.configuration.rules {
                    println!("Checking rule: {:?}", rule);
                    let rule_result = rule.check(&record);
                    match rule_result {
                        Ok(result) => {
                            println!("Rule result: {:?}", result);
                            match result {
                                RuleResult::Pass => {continue;},
                                RuleResult::Fail => {return rule.behaviour_on_violation.clone();},
                            }
                        }
                        Err(e) => {
                            println!("Unexpected error on rule checking analyzing: {:?}", e);
                            return self.configuration.behaviour_on_incidents.clone();
                        }
                    }
                }
                Behaviour::LogOnly
            }
        }
    }
}