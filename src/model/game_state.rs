#[derive(Debug, Clone, PartialEq)]
pub struct GameState {
    inventory: Vec<String>,
    scene_name: String,
    scene_desc: String,
    user_entry: String,
    entry_enabled: bool,
    scene_history: Vec<String>,
    scroll_position: usize,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            inventory: Vec::new(),
            scene_name: String::from("New Game"),
            scene_desc: String::from("You are in a dark room. There is a door to the north. There is a door to the south. There is a door to the east. There is a door to the west. There is a door to the up. There is a door to the down. There is a door to the northeast. There is a door to the northwest. There is a door to the southeast. There is a door to the southwest. There is a door to the in. There is a door to the out. There is a door to the left."),
            user_entry: String::new(),
            entry_enabled: true,
            scene_history: Vec::new(),
            scroll_position: 0,
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

    pub fn push_input_to_history(&mut self) {
        self.append_scene_history(format!("> {}", self.user_entry.clone()));
        self.user_entry.clear();
    }

    pub fn append_scene_history(&mut self, s: String) {
        self.scene_history.push(s);
        self.scroll_reset();
    }

    pub fn scroll_up(&mut self, amount: usize) {
        self.scroll_position += amount;
    }

    pub fn scroll_down(&mut self, amount: usize) {
        if amount > self.scroll_position {
            self.scroll_position = 0;
            return;
        }
        self.scroll_position -= amount;
    }

    pub fn scroll_reset(&mut self) {
        self.scroll_position = 0;
    }

    pub fn new_scene(&mut self, name: String, desc: String) {
        self.scene_name = name;
        self.scene_desc = desc;
        self.entry_enabled = true;
        self.scene_history.clear();
        self.scroll_reset();
    }

    pub fn add_to_inventory(&mut self, item: String) {
        self.inventory.push(item);
    }

    pub fn remove_from_inventory(&mut self, item: String) {
        self.inventory.retain(|i| i != &item);
    }

    pub fn disable_entry(&mut self) {
        self.entry_enabled = false;
    }

    // Getters

    pub fn get_scroll_position(&self) -> usize {
        self.scroll_position
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
