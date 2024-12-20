use std::sync::Arc;
use tokio::sync::{RwLock, broadcast};
use super::SystemState;
use crate::error::{ElizaCliError, Result};

#[derive(Clone)]
pub struct SharedState {
    state: Arc<RwLock<SystemState>>,
    update_tx: broadcast::Sender<()>,
}

impl SharedState {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(100);
        Self {
            state: Arc::new(RwLock::new(SystemState::default())),
            update_tx: tx,
        }
    }

    pub async fn read_state(&self) -> SystemState {
        let guard = self.state.read().await;
        guard.clone()
    }

    pub async fn update<F>(&self, mut updater: F) -> Result<()>
    where
        F: FnOnce(&mut SystemState) -> std::result::Result<(), std::io::Error>,
    {
        {
            let mut guard = self.state.write().await;
            updater(&mut guard).map_err(ElizaCliError::IoError)?;
        }
        let _ = self.update_tx.send(()); // notificaiton to the subscribers 
        Ok(())
    }

    pub fn subscribe(&self) -> broadcast::Receiver<()> {
        self.update_tx.subscribe()
    }

    pub async fn add_agent(&self, agent: super::AgentState) -> Result<()> {
        self.update(|state| {
            state.agents.push(agent);
            Ok(())
        }).await
    }

    pub async fn add_client(&self, client: super::ClientState) -> Result<()> {
        self.update(|state| {
            state.clients.push(client);
            Ok(())
        }).await
    }

    pub async fn set_metrics(&self, cpu: f64, mem: f64) -> Result<()> {
        self.update(|state| {
            state.metrics.cpu_usage = cpu;
            state.metrics.memory_usage = mem;
            Ok(())
        }).await
    }
}