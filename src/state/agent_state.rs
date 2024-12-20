#[derive(Debug, Clone)]
pub struct AgentState {
    pub id: String, 
    pub name: String,
    pub model_provider: String,
}

impl AgentState {
    pub fn new(id: &str, name: &str, model_provider: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            model_provider: model_provider.to_string(),
        }
    }
}