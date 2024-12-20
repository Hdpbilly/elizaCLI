#[derive(Debug, Clone)]
pub struct PluginState {
    pub name: String,
    pub version: String,
    pub active: bool,
}

impl PluginState {
    pub fn new(name: &str, version: &str, active: bool) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
            active,
        }
    }
}