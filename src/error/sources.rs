#[derive(Debug)]
pub struct ParseMsg(pub &'static str);

impl std::fmt::Display for ParseMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ParseMsg {}

#[derive(Debug)]
pub struct ChronoDateError(pub chrono::ParseError);

impl std::fmt::Display for ChronoDateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::error::Error for ChronoDateError {}
