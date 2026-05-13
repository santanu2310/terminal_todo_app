use crate::app::App;
use ratatui::{Frame, widgets::Block};

pub fn render(f: &mut Frame, app: &App) {
    let block = Block::bordered().title("Bordered block");
    f.render_widget(block, f.area());
}
