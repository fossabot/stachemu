#[derive(PartialEq, Debug, Clone)]
pub enum Rule {
    Symbolic(String, String),
    Default(String)
}
