use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Condition {
    MustBeIn,
    MustNotBeIn,
}
