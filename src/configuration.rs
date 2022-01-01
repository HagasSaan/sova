#[derive(Clone)]
pub enum BehaviourOnIncidents {
    LogOnly,
    BasedOnAnalysisResult,
    KillProcess,
    KillSystem,
}

#[derive(Clone)]
pub struct Configuration {
    pub unix_socket_path: String,
    pub behaviour_on_incidents: BehaviourOnIncidents,
}

impl Configuration {
    pub fn new(
        unix_socket_path: String,
        behaviour_on_incidents: BehaviourOnIncidents,
    ) -> Configuration {
        Configuration {
            unix_socket_path,
            behaviour_on_incidents,
        }
    }
}