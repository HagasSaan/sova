#[derive(Debug)]
pub struct Record {
    pub pathname: String,
    pub argv: Option<Vec<String>>,
}
