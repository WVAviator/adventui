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
            scene_desc: String::from("You are in a dark room. There is a door to the north. There is a door to the south. There is a door to the east. There is a door to the west. There is a door to the up. There is a door to the down. There is a door to the northeast. There is a door to the northwest. There is a door to the southeast. There is a door to the southwest. There is a door to the in. There is a door to the out. There is a door to the left."),
            user_entry: String::new(),
            scene_history: Vec::new(),
        }
    }

    pub fn append_entry(&mut self, c: char) {
        if self.user_entry.len() >= 100 {
            return;
        }
        self.user_entry += &c.to_string();
    }

    pub fn remove_last_entry(&mut self) {
        self.user_entry.pop();
    }

    pub fn process_input(&mut self) {
        self.scene_history
            .push(format!("> {}", self.user_entry.clone()));
        self.user_entry.clear();
    }

    pub fn get_user_entry(&self) -> &str {
        &self.user_entry
    }

    pub fn get_scene_title(&self) -> &str {
        &self.scene_name
    }

    pub fn get_scene_desc(&self) -> &str {
        &self.scene_desc
    }

    pub fn get_scene_history(&self) -> &Vec<String> {
        &self.scene_history
    }

    pub fn get_inventory(&self) -> &Vec<String> {
        &self.inventory
    }
}
