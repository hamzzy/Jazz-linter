use serde::Serialize;

#[derive(Serialize)]
pub struct Issue {
    pub rule: String,
    pub message: String,
}