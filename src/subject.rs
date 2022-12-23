use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Subject {
    Pathname,  // execv, execve, open
    Argv,  // execv, execve
}
