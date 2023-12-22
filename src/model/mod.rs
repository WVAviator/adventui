use self::{game_state::GameState, main_menu_state::MainMenuState};

pub mod game_state;
pub mod main_menu_state;

#[derive(Debug, Clone, PartialEq)]
pub enum Model {
    MainMenu(MainMenuState),
    Game(GameState),
}

impl Model {
    pub fn new() -> Self {
        Model::MainMenu(MainMenuState::new())
    }
}
