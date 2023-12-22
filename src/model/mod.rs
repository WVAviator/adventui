use self::{game_state::GameState, main_menu_state::MainMenuState};

mod game_state;
mod main_menu_state;

pub enum Model {
    MainMenu(MainMenuState),
    Game(GameState),
}

impl Model {
    pub fn new() -> Self {
        Model::MainMenu(MainMenuState::new())
    }
}
