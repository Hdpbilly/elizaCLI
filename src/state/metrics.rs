#[derive(Debug, Clone)]
pub struct Metrics {
    pub cpu_usage: f64,
    pub memory_usage: f64,
}

impl Default for Metrics {
    fn default() -> Self {
        Self {
            cpu_usage: 0.0,
            memory_usage: 0.0,
        }
    }
}