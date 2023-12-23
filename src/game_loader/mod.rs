// The game loader is responsible for loading responses to each user input and logically determine what actions should result

use crate::action::Action;

pub struct GameLoader {}

impl GameLoader {
    pub fn new() -> Self {
        GameLoader {}
    }

    pub fn process_input(&self, input: &str) -> Action {
        match input {
            "north" => Action::NewScene {
                name: String::from("North"),
                desc: String::from("You are in the north room."),
            },
            "get key" => Action::AddToInventory {
                item: String::from("Key"),
                message: String::from("You picked up a key."),
            },
            "use key" => Action::RemoveFromInventory {
                item: String::from("Key"),
                message: String::from("You used a key. It is now gone."),
            },
            "window" => Action::Information {
                message: String::from("You look out the window. It is dark."),
            },
            "die" => Action::EndGame {
                message: String::from("You died."),
            },
            _ => Action::Information {
                message: String::from("I don't understand."),
            },
        }
    }
}
