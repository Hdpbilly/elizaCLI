// tests/state_tests.rs
use eliza_cli::state::{SharedState, AgentState, ClientState};
use tokio::sync::broadcast;
use eliza_cli::error::Result;

#[tokio::test]
async fn test_default_state() -> Result<()> {
    let shared = SharedState::new();
    let state = shared.read_state().await;
    assert!(state.agents.is_empty());
    assert!(state.clients.is_empty());
    assert!(state.plugins.is_empty());
    Ok(())
}

#[tokio::test]
async fn test_add_agent() -> Result<()> {
    let shared = SharedState::new();
    let agent = AgentState::new("123", "test agent", "llama haha");
    shared.add_agent(agent.clone()).await?;

    let state = shared.read_state().await;
    assert_eq!(state.agents.len(), 1);
    assert_eq!(state.agents[0].id, "123");
    assert_eq!(state.agents[0].name, "test agent");
    assert_eq!(state.agents[0].model_provider, "llama haha");
    Ok(())
}

#[tokio::test]
async fn test_broadcast_on_update() -> Result<()> {
    let shared = SharedState::new();
    let mut rx = shared.subscribe();

    assert!(rx.try_recv().is_err(), "no update has happend yet");

    shared.add_client(ClientState::new("Discord", "Running")).await?;

    // check notif
    let notification = rx.recv().await;
    assert!(notification.is_ok(), "update notification should have been recieved");

    let state = shared.read_state().await;
    assert_eq!(state.clients.len(), 1);
    assert_eq!(state.clients[0].name, "Discord"); 
    assert_eq!(state.clients[0].status, "Running"); 
    Ok(())
}

#[tokio::test]
async fn test_set_metrics() -> Result<()> {
    let shared = SharedState::new();
    shared.set_metrics(50.0, 1527.0).await?;
    let state = shared.read_state().await;
    assert_eq!(state.metrics.cpu_usage, 50.0);
    assert_eq!(state.metrics.memory_usage, 1527.0);
    Ok(())
    
}
