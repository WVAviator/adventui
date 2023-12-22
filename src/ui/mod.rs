use std::{
    io::Stdout,
    sync::{mpsc::Receiver, Arc, Mutex},
    thread::JoinHandle,
};

use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{CrosstermBackend, Stylize, Terminal},
    style::Color,
    text::Text,
    widgets::{Block, Borders, List, ListItem, Paragraph},
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
                    terminal
                        .draw(|frame| {
                            let area = frame.size();
                            frame.render_widget(
                                Paragraph::new(state.get_user_entry()).white(),
                                area,
                            );
                        })
                        .expect("Failed to draw game frame.");
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
