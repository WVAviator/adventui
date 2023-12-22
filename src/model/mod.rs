use self::{main_menu_state::MainMenuState, game_state::GameState};

mod game_state;
mod main_menu_state;

pub enum Model<'a> {
    MainMenu(MainMenuState<'a>),
    Game(GameState<'a>)
}

impl<'a> Model<'a> {
    pub fn new() -> Self {
        Model::MainMenu(MainMenuState::new())w
    }
}
