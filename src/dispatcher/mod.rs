use crossterm::event::{self, KeyCode, KeyEventKind};
use std::{sync::mpsc::Sender, thread::JoinHandle};

use crate::{
    message::Message,
    model::{game_state::GameState, Model},
};

pub struct Dispatcher {
    join_handle: Option<JoinHandle<()>>,
}

impl Dispatcher {
    pub fn new(app_state_tx: Sender<Message>, ui_state_tx: Sender<Message>) -> Self {
        let join_handle = std::thread::spawn(move || {
            let mut model = Model::new();
            ui_state_tx
                .send(Message::StateUpdate(model.clone()))
                .expect("Failed to initialize application state for UI.");
            loop {
                if let event::Event::Key(key) =
                    event::read().expect("Failed to read event keypress.")
                {
                    match &mut model {
                        Model::MainMenu(state) => {
                            if key.kind == KeyEventKind::Press {
                                match key.code {
                                    KeyCode::Char('j') | KeyCode::Down => {
                                        state.select_next();
                                    }
                                    KeyCode::Char('k') | KeyCode::Up => {
                                        state.select_prev();
                                    }
                                    KeyCode::Enter => match state.get_selection() {
                                        "New Game" => {
                                            model = Model::Game(GameState::new());
                                        }
                                        _ => {
                                            unimplemented!("Main menu option not implemented yet.")
                                        }
                                    },
                                    KeyCode::Esc => {
                                        ui_state_tx
                                            .send(Message::Terminate)
                                            .expect("Failed to send UI terminate message.");
                                        app_state_tx.send(Message::Terminate).expect(
                                            "Failed to send application terminate message.",
                                        );
                                        break;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        Model::Game(state) => {
                            if key.kind == KeyEventKind::Press {
                                match key.code {
                                    KeyCode::Char(c) => {
                                        state.append_entry(c);
                                    }
                                    KeyCode::Backspace => {
                                        state.remove_last_entry();
                                    }
                                    KeyCode::Enter => {
                                        state.process_input();
                                    }
                                    KeyCode::Up => {
                                        state.scroll_up(1);
                                    }
                                    KeyCode::Down => {
                                        state.scroll_down(1);
                                    }
                                    KeyCode::PageUp => {
                                        state.scroll_up(10);
                                    }
                                    KeyCode::PageDown => {
                                        state.scroll_down(10);
                                    }
                                    KeyCode::Esc => {
                                        ui_state_tx
                                            .send(Message::Terminate)
                                            .expect("Failed to send UI terminate message.");
                                        app_state_tx.send(Message::Terminate).expect(
                                            "Failed to send application terminate message.",
                                        );
                                        break;
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }

                    ui_state_tx
                        .send(Message::StateUpdate(model.clone()))
                        .expect("Failed to send updated state to UI.");
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
