use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Behaviour {
    KillSystem,
    KillProcess,
    LogOnly,
}
