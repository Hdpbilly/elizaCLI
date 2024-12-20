use eliza_cli::{
    init_logger,
    ui::{App, init_terminal, restore_terminal, draw_ui, Events, AppEvent},
};
use tracing::info;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    init_logger();
    info!("Starting Eliza CLI...");

    let mut terminal = init_terminal()?;
    let mut app = App::new();

    let events = Events::new(std::time::Duration::from_millis(250));

    // Main event loop
    while app.is_running() {
        draw_ui(&app, &mut terminal);

        match events.next_event()? {
            AppEvent::Input(key) => {
                match key {
                    crossterm::event::KeyCode::Char('q') => {
                        app.stop();
                    }
                    crossterm::event::KeyCode::Char('c') if crossterm::event::KeyModifiers::CONTROL == crossterm::event::KeyModifiers::CONTROL => {
                        app.stop();
                    }
                    other => {
                        info!("Key pressed: {:?}", other);
                    }
                }
            }
            AppEvent::Resize(_w, _h) => {
                // Just redraw on next loop iteration.
                info!("Terminal resized");
            }
            AppEvent::Tick => {
                // Could handle periodic tasks here.
            }
        }
    }

    restore_terminal(terminal)?;
    info!("Eliza CLI shutting down.");

    Ok(())
}
