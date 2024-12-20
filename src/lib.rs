// src/lib.rs
pub mod error;
pub mod config;

pub mod commands;
pub mod data;
pub mod plugins;
pub mod state;
pub mod ui;

use tracing_subscriber::{fmt, EnvFilter};
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub fn init_logger() {
    let filter = EnvFilter::from_default_env()
        .add_directive("eliza-cli=info".parse().unwrap());

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(filter)
        .init();

    tracing::info!("Logger initialized");
}
