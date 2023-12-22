#[derive(Debug, Clone, PartialEq)]
pub struct GameState {
    inventory: Vec<String>,
    scene_name: String,
    scene_desc: String,
    user_entry: String,
    scene_history: Vec<String>,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            inventory: Vec::new(),
            scene_name: String::from("New Game"),
            scene_desc: String::from("Loading..."),
            user_entry: String::new(),
            scene_history: Vec::new(),
        }
    }
}
