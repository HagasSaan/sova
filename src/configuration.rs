use behaviour::Behaviour;
use rule::Rule;

pub struct Configuration {
    pub behaviour_on_incidents: Behaviour,
    pub rules: Vec<Rule>,
}
