pub struct MainMenuState<'a> {
    options: Vec<&'a str>,
    selection: usize,
}

impl<'a> MainMenuState<'a> {
    pub fn new() -> Self {
        MainMenuState {
            options: vec!["New Game", "Continue", "Settings", "Quit"],
            selection: 0,
        }
    }
}
