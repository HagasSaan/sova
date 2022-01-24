use behaviour::Behaviour;
use rule::Rule;

pub struct Configuration {
    pub behaviour_on_incidents: Behaviour,
    pub rules: Vec<Rule>,
}

impl Configuration {
    pub fn load(path: &str) -> Result<Self, &str> {
        unimplemented!();
    }

    pub fn dump(path: &str) -> Result<(), &str> {
        unimplemented!();
    }
}