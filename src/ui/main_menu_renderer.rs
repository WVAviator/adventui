use std::{
    io::Stdout,
    sync::{Arc, Mutex},
};

use ratatui::{
    prelude::{CrosstermBackend, Terminal},
    style::Color,
    text::Text,
    widgets::{Block, Borders, List, ListItem},
};

use crate::model::main_menu_state::MainMenuState;

pub struct MainMenuRenderer {
    terminal: Arc<Mutex<Terminal<CrosstermBackend<Stdout>>>>,
}

impl MainMenuRenderer {
    pub fn new(terminal: Arc<Mutex<Terminal<CrosstermBackend<Stdout>>>>) -> Self {
        MainMenuRenderer { terminal }
    }

    pub fn render(&self, state: MainMenuState) {
        let mut terminal = self
            .terminal
            .lock()
            .expect("Unable to get lock on terminal for UI rendering.");
        terminal
            .draw(|frame| {
                let items: Vec<ListItem> = state
                    .get_options()
                    .iter()
                    .enumerate()
                    .map(|(i, item)| {
                        if i == state.get_selection_index() {
                            ListItem::new(Text::styled(
                                item,
                                ratatui::style::Style::default().fg(Color::Yellow), // Highlight style
                            ))
                        } else {
                            ListItem::new(Text::raw(item))
                        }
                    })
                    .collect();

                let list = List::new(items).block(Block::default().borders(Borders::ALL)); // Optional border

                frame.render_widget(list, frame.size());
            })
            .expect("Failed to draw menu frame.");
    }
}
