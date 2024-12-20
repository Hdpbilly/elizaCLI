// src/state module
mod shared_state;
mod agent_state;
mod client_state;
mod plugin_state;
mod metrics;

pub use shared_state::SharedState;
pub use agent_state::AgentState;
pub use client_state::ClientState;
pub use plugin_state::PluginState;
pub use metrics::Metrics;

use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct SystemState {
    pub agents: Vec<AgentState>,
    pub clients: Vec<ClientState>,
    pub plugins: Vec<PluginState>,
    pub metrics: Metrics,
}

impl Default for SystemState {
    fn default() -> Self {
        Self {
            agents: vec![],
            clients: vec![],
            plugins: vec![],
            metrics: Metrics::default(),
        }
    }
}
