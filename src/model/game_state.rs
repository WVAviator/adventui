pub struct GameState<'a> {
    inventory: Vec<&'a str>,
    scene_name: &'a str,
    scene_desc: &'a str,
    user_entry: String,
    scene_history: Vec<&'a str>,
}

impl<'a> GameState<'a> {
    pub fn new() -> Self {
        GameState {
            inventory: Vec::new(),
            scene_name: "New Game",
            scene_desc: "Loading...",
            user_entry: String::new(),
            scene_history: Vec::new(),
        }
    }
}
