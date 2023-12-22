use self::main_menu_state::MainMenuState;

mod main_menu_state;

pub enum Model<'a> {
    MainMenu(MainMenuState<'a>),
}

impl<'a> Model<'a> {
    pub fn new() -> Self {
        Model::MainMenu(MainMenuState::new())
    }
}
