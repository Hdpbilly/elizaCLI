// src/ui module
pub mod app;
pub mod events;
mod styles; 

pub use app::{App, init_terminal, restore_terminal};
pub use events::{Events, AppEvent};

use ratatui::{Frame, widgets::{Block, Borders}, layout::{Layout, Direction, Constraint}};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

pub fn draw_ui(app: &App, terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) {
    terminal
        .draw(|f| {
            render_layout(f, app);
        })
        .expect("Failed to draw UI");
}

fn render_layout(frame: &mut Frame, _app: &App) {
    let size = frame.size();
    let block = Block::default()
        .title("Eliza CLI - Test Block")
        .borders(Borders::ALL);

    frame.render_widget(block, size);
}