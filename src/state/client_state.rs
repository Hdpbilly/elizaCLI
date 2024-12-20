#[derive(Debug, Clone)]
pub struct ClientState {
    pub name: String, 
    pub status: String,
}

impl ClientState {
    pub fn new(name: &str, status: &str) -> Self {
        Self {
            name: name.to_string(),
            status: status.to_string(),
        }
    }
}