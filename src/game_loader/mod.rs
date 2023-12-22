// The game loader is responsible for loading responses to each user input and logically determine what actions should result

pub struct GameLoader {}

impl GameLoader {
    pub fn new() -> Self {
        GameLoader {}
    }

    pub fn process_input(&self, input: &str) -> String {
        match input {
            "look" => "You are in a dark room. There is a door to the north. There are large windows to the south. There is a door to the east. There is a door to the west. There is a door to the up. There is a door to the down. There is a door to the northeast. There is a door to the northwest.".to_string(),
            "north" => "You go north.".to_string(),
            "south" => "You go south.".to_string(),
            "east" => "You go east.".to_string(),
            "west" => "You go west.".to_string(),
            _ => "I don't understand.".to_string(),
        }
    }
}
