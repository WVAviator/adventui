use std::{
    io::Stdout,
    sync::{mpsc::Receiver, Arc, Mutex},
    thread::JoinHandle,
};

use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
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
                Ok(Message::StateUpdate(Model::MainMenu(model))) => {
                    let mut terminal = terminal
                        .lock()
                        .expect("Unable to get lock on terminal for UI rendering.");
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
                Ok(Message::StateUpdate(Model::Game(model))) => {
                    let mut terminal = terminal
                        .lock()
                        .expect("Unable to get lock on terminal for UI rendering.");
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
