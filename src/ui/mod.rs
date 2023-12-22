use std::{io::stdout, sync::mpsc::Receiver, thread::JoinHandle};

use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};

use crate::model::Model;

pub struct UI {
    join_handle: Option<JoinHandle<()>>,
}

impl UI {
    pub fn new(model_update_rx: Receiver<Model>) -> Self {
        let mut terminal =
            Terminal::new(CrosstermBackend::new(stdout())).expect("Unable to create terminal UI.");
        terminal.clear().expect("Failed to clear terminal.");

        let join_handle = std::thread::spawn(move || loop {
            match model_update_rx.recv() {
                Ok(Model::MainMenu(model)) => {
                    terminal
                        .draw(|frame| {
                            let area = frame.size();
                            frame.render_widget(
                                Paragraph::new("Main menu! Yay!").white().on_blue(),
                                area,
                            );
                        })
                        .expect("Failed to draw menu frame.");
                }
                Ok(Model::Game(model)) => {
                    terminal
                        .draw(|frame| {
                            let area = frame.size();
                            frame.render_widget(
                                Paragraph::new("Game! Yay!").white().on_blue(),
                                area,
                            );
                        })
                        .expect("Failed to draw game frame.");
                }
                _ => {}
            }
        });

        UI {
            join_handle: Some(join_handle),
        }
    }
}
