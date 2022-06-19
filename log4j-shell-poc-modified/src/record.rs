#[derive(Debug)]
pub struct Record {
    pub path: String,
    pub argv: Option<Vec<String>>,
    pub envp: Option<Vec<String>>,
}
