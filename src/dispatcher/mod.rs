use crossterm::event::{self, KeyCode, KeyEventKind};
use std::{sync::mpsc::Sender, thread::JoinHandle};

use crate::{
    message::Message,
    model::{game_state::GameState, main_menu_state::MainMenuState, Model},
};

pub struct Dispatcher {
    join_handle: Option<JoinHandle<()>>,
}

impl Dispatcher {
    pub fn new(app_state_tx: Sender<Message>, ui_state_tx: Sender<Message>) -> Self {
        let join_handle = std::thread::spawn(move || {
            let mut model = Model::new();
            loop {
                if let event::Event::Key(key) =
                    event::read().expect("Failed to read event keypress.")
                {
                    if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('m') {
                        model = Model::MainMenu(MainMenuState::new());
                        ui_state_tx
                            .send(Message::StateUpdate(model.clone()))
                            .expect("Failed to dispatch model update.");
                    }
                    if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('g') {
                        model = Model::Game(GameState::new());
                        ui_state_tx
                            .send(Message::StateUpdate(model.clone()))
                            .expect("Failed to dispatch model update.");
                    }
                    if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                        ui_state_tx
                            .send(Message::Terminate)
                            .expect("Failed to send UI terminate message.");
                        app_state_tx
                            .send(Message::Terminate)
                            .expect("Failed to send application terminate message.");
                        break;
                    }
                }
            }
        });

        Dispatcher {
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
