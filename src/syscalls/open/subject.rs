use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Subject {
    Pathname,
    PathnameAndFlags,
}
