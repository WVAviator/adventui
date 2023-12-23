use crossterm::event::{self, KeyCode, KeyEventKind};
use std::{sync::mpsc::Sender, thread::JoinHandle};

use crate::{
    action::Action,
    game_loader::GameLoader,
    message::Message,
    model::{game_state::GameState, Model},
};

pub struct Dispatcher {
    join_handle: Option<JoinHandle<()>>,
}

impl Dispatcher {
    pub fn new(
        app_state_tx: Sender<Message>,
        ui_state_tx: Sender<Message>,
        mut loader: GameLoader,
    ) -> Self {
        let join_handle = std::thread::spawn(move || {
            let mut model = Model::new();
            let mut is_new_game = false;
            ui_state_tx
                .send(Message::StateUpdate(model.clone()))
                .expect("Failed to initialize application state for UI.");
            loop {
                if is_new_game {
                    is_new_game = false;
                    if let Model::Game(state) = &mut model {
                        loader.create_game();
                        let action = loader.process_input("start game", state);
                        if let Action::NewScene { name, desc } = action {
                            state.new_scene(name, desc);
                            state.enable_entry();
                        }
                    }
                    ui_state_tx
                        .send(Message::StateUpdate(model.clone()))
                        .expect("Failed to send updated state to UI.");
                    continue;
                }
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
                                            is_new_game = true;
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
                                        state.disable_entry();
                                        let action: Action =
                                            loader.process_input(&state.get_user_entry(), &state);
                                        state.push_input_to_history();
                                        match action {
                                            Action::NewScene { name, desc } => {
                                                state.new_scene(name, desc);
                                                state.enable_entry();
                                            }
                                            Action::AddToInventory { item, message } => {
                                                state.add_to_inventory(item);
                                                state.append_scene_history(message);
                                                state.enable_entry();
                                            }
                                            Action::RemoveFromInventory { item, message } => {
                                                state.remove_from_inventory(item);
                                                state.append_scene_history(message);
                                                state.enable_entry();
                                            }
                                            Action::Information { message } => {
                                                state.append_scene_history(message);
                                                state.enable_entry();
                                            }
                                            Action::EndGame { message } => {
                                                state.append_scene_history(message);
                                                state.disable_entry();
                                            }
                                        }
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
