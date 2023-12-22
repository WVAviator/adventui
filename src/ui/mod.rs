use std::{
    io::{self, Stdout},
    sync::{mpsc::Receiver, Arc, Mutex},
    thread::JoinHandle,
};

use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{CrosstermBackend, Terminal},
    style::Color,
    text::Text,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crossterm::{
    cursor::{EnableBlinking, MoveTo, SetCursorStyle, Show},
    execute,
};

use crate::{message::Message, model::Model};

pub struct UI {
    join_handle: Option<JoinHandle<()>>,
}

impl UI {
    pub fn new(
        model_update_rx: Receiver<Message>,
        terminal: Arc<Mutex<Terminal<CrosstermBackend<Stdout>>>>,
    ) -> Self {
        let join_handle = std::thread::spawn(move || loop {
            match model_update_rx.recv() {
                Ok(Message::StateUpdate(Model::MainMenu(state))) => {
                    let mut terminal = terminal
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

                            let list =
                                List::new(items).block(Block::default().borders(Borders::ALL)); // Optional border

                            frame.render_widget(list, frame.size());
                        })
                        .expect("Failed to draw menu frame.");
                }
                Ok(Message::StateUpdate(Model::Game(state))) => {
                    let mut terminal = terminal
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
                Ok(Message::Terminate) => {
                    break;
                }
                _ => {}
            }
        });

        UI {
            join_handle: Some(join_handle),
        }
    }

    pub fn close(&mut self) -> Result<(), ()> {
        self.join_handle
            .take()
            .unwrap()
            .join()
            .expect("Unable to join thread.");
        Ok(())
    }
}
