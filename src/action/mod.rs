use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum Action {
    NewScene { name: String, desc: String },
    AddToInventory { item: String, message: String },
    RemoveFromInventory { item: String, message: String },
    Information { message: String },
    EndGame { message: String },
}

impl Action {
    pub fn from_yaml(yaml: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(yaml)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn action_deserialize_new_scene() {
        let yaml = r#"
            type: NewScene
            name: Test Scene
            desc: This is a test scene.
        "#;

        let expected = Action::NewScene {
            name: String::from("Test Scene"),
            desc: String::from("This is a test scene."),
        };

        let actual: Action = Action::from_yaml(yaml).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn action_deserialize_add_to_inventory() {
        let yaml = r#"
            type: AddToInventory
            item: Test Item
            message: You picked up a test item.
        "#;

        let expected = Action::AddToInventory {
            item: String::from("Test Item"),
            message: String::from("You picked up a test item."),
        };

        let actual: Action = Action::from_yaml(yaml).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn action_deserialize_remove_from_inventory() {
        let yaml = r#"
            type: RemoveFromInventory
            item: Test Item
            message: You dropped a test item.
        "#;

        let expected = Action::RemoveFromInventory {
            item: String::from("Test Item"),
            message: String::from("You dropped a test item."),
        };

        let actual: Action = Action::from_yaml(yaml).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn action_deserialize_information() {
        let yaml = r#"
            type: Information
            message: This is an information message.
        "#;

        let expected = Action::Information {
            message: String::from("This is an information message."),
        };

        let actual: Action = Action::from_yaml(yaml).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn action_deserialize_end_game() {
        let yaml = r#"
            type: EndGame
            message: This is an end game message.
        "#;

        let expected = Action::EndGame {
            message: String::from("This is an end game message."),
        };

        let actual: Action = Action::from_yaml(yaml).unwrap();

        assert_eq!(actual, expected);
    }
}
