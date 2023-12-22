use std::{
    io::{self, Stdout},
    sync::{Arc, Mutex},
};

use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{CrosstermBackend, Terminal},
    style::Color,
    widgets::{Block, Borders, Paragraph},
};

use crossterm::{
    cursor::{EnableBlinking, MoveTo, SetCursorStyle, Show},
    execute,
};

use crate::model::game_state::GameState;

pub struct GameRenderer {
    terminal: Arc<Mutex<Terminal<CrosstermBackend<Stdout>>>>,
}

impl GameRenderer {
    pub fn new(terminal: Arc<Mutex<Terminal<CrosstermBackend<Stdout>>>>) -> Self {
        GameRenderer { terminal }
    }

    pub fn render(&self, state: GameState) {
        let mut terminal = self
            .terminal
            .lock()
            .expect("Unable to get lock on terminal for UI rendering.");
        let margin = 2;
        terminal
            .draw(|frame| {
                let size = frame.size();
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(margin)
                    .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
                    .split(size);
                frame.render_widget(
                    Paragraph::new(state.get_user_entry())
                        .style(ratatui::style::Style::default().fg(Color::White))
                        .block(Block::default().borders(Borders::ALL).title("Input")),
                    chunks[0],
                );
            })
            .expect("Failed to draw game frame.");

        // Moves the position and display style of the terminal cursor to match the input.
        execute!(
            io::stdout(),
            MoveTo(margin + 1 + state.get_user_entry().len() as u16, margin + 1),
            EnableBlinking,
            Show,
            SetCursorStyle::BlinkingBar
        )
        .expect("Unable to reposition cursor for user input.")
    }
}
