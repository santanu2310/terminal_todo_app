use crate::app::App;
use ratatui::crossterm::event::{KeyCode, KeyEvent};

pub fn handle_key_events(
    key_event: KeyEvent,
    app: &mut App,
) -> Result<(), Box<dyn std::error::Error>> {
    match key_event.code {
        KeyCode::Char('q') => app.running = false,
        _ => {}
    }
    Ok(())
}
