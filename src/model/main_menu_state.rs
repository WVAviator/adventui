#[derive(Debug, Clone, PartialEq)]
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

    pub fn select_next(&mut self) {
        self.selection += 1;
        self.selection %= self.options.len();
    }

    pub fn select_prev(&mut self) {
        self.selection += self.options.len();
        self.selection -= 1;
        self.selection %= self.options.len();
    }

    pub fn get_selection(&self) -> &str {
        &self.options[self.selection]
    }

    pub fn get_options(&self) -> &Vec<String> {
        &self.options
    }

    pub fn get_selection_index(&self) -> usize {
        self.selection
    }
}
