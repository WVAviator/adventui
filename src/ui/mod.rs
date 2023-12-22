use std::{
    io::stdout,
    sync::{mpsc::Receiver, Arc, Mutex},
    thread::JoinHandle,
};

use ratatui::prelude::{CrosstermBackend, Terminal};

use crate::{message::Message, model::Model};

use self::{game_renderer::GameRenderer, main_menu_renderer::MainMenuRenderer};

mod game_renderer;
mod main_menu_renderer;

pub struct UI {
    join_handle: Option<JoinHandle<()>>,
}

impl UI {
    pub fn new(model_update_rx: Receiver<Message>) -> Self {
        let mut terminal =
            Terminal::new(CrosstermBackend::new(stdout())).expect("Unable to create terminal UI.");
        terminal.clear().expect("Failed to clear terminal.");

        let terminal = Arc::new(Mutex::new(terminal));

        let main_menu_renderer = MainMenuRenderer::new(terminal.clone());
        let game_renderer = GameRenderer::new(terminal.clone());

        let join_handle = std::thread::spawn(move || loop {
            match model_update_rx.recv() {
                Ok(Message::StateUpdate(Model::MainMenu(state))) => {
                    main_menu_renderer.render(state);
                }
                Ok(Message::StateUpdate(Model::Game(state))) => {
                    game_renderer.render(state);
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
