pub struct MainMenuState {
    options: Vec<String>,
    selection: usize,
}

impl MainMenuState {
    pub fn new() -> Self {
        MainMenuState {
            options: vec![
                String::from("New Game"),
                String::from("Continue"),
                String::from("Settings"),
                String::from("Quit"),
            ],
            selection: 0,
        }
    }
}
