// The game loader is responsible for loading responses to each user input and logically determine what actions should result

use std::fs::{self, File, OpenOptions};
use std::io::Write;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{action::Action, model::game_state::GameState};

pub struct GameLoader {
    api_key: String,
    overview: String,
    action_history: Vec<ActionHistoryItem>,
    system_prompt: String,
    log: File,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Context {
    overview: String,
    inventory: Vec<String>,
    history: Vec<ActionHistoryItem>,
    input: String,
}

impl Context {
    pub fn new(
        overview: String,
        inventory: Vec<String>,
        history: Vec<ActionHistoryItem>,
        input: String,
    ) -> Self {
        Context {
            overview,
            inventory,
            history,
            input,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ActionHistoryItem {
    input: String,
    response: Action,
}

impl ActionHistoryItem {
    pub fn new(input: String, response: Action) -> Self {
        ActionHistoryItem { input, response }
    }
}

impl GameLoader {
    pub fn new() -> Self {
        let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set.");
        let system_prompt = fs::read_to_string("prompt.txt").expect("Failed to read prompt.txt");
        let log = OpenOptions::new()
            .write(true)
            .append(true)
            .open("log.txt")
            .expect("Failed to open log.txt");

        GameLoader {
            api_key,
            overview: String::new(),
            action_history: Vec::new(),
            system_prompt,
            log,
        }
    }

    pub fn create_game(&mut self) {
        let client = reqwest::blocking::Client::new();

        let body = json!({
            "model": "gpt-4-1106-preview",
            "messages": [
                {
                    "role": "system",
                    "content": "You are a text adventure game designer. Your job is to come up with a new game idea that would work well as a text adventure game, and provide a single-paragraph overview of the setting, the goal, characters, and any rules. This overview will be used in subsequent requests to a less-powerful LLM as part of the context for generating parts of the game, so be sure to provide enough information, but not too much detail, so that a weaker LLM can remain focused with its gameplay narratives. Some potential topics might include a fantasy adventure, a sci-fi adventure, a mystery, or a horror story. Some example settings might include a desert, spaceship, castle, or haunted house. Some example goals might include finding a treasure, escaping a monster, traveling between planets in space, or solving a mystery. Some example characters might include a shopkeeper, ship captain, companion, or ghost. Some example rules might include magic, technology, or a curse."
                },
                {
                    "role": "user",
                    "content": "please provide a game overview"
                }
            ]
        });

        writeln!(
            self.log,
            "Sending request to OpenAI\n--------\n{}",
            serde_json::to_string_pretty(&body).unwrap()
        )
        .unwrap();
        writeln!(self.log, "--------").unwrap();

        let response = client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .body(body.to_string())
            .send()
            .expect("Failed to send request.");

        // Check if the response status is success
        if response.status().is_success() {
            let response_json: Value = response.json().expect("Failed to parse response.");
            let mut response_text = response_json["choices"][0]["message"]["content"]
                .as_str()
                .expect("Failed to parse response.");
            self.overview = response_text.to_string();
            writeln!(
                self.log,
                "Received response from OpenAI\n--------\n{:?}",
                &response_text
            )
            .unwrap();
            writeln!(self.log, "--------").unwrap();
        } else {
            panic!(
                "Failed to get a successful response: {:?}",
                response.status()
            );
        }
    }

    pub fn process_input(&mut self, input: &str, state: &GameState) -> Action {
        let context = Context::new(
            self.overview.clone(),
            state.get_inventory().clone(),
            self.action_history.clone(),
            String::from(input),
        );
        let context = serde_yaml::to_string(&context).unwrap();
        let action = self.send_openai_request(&context).unwrap();
        self.add_action_to_history(action.clone());
        action
    }

    fn add_action_to_history(&mut self, action: Action) {
        let history_limit = 12;
        self.action_history
            .push(ActionHistoryItem::new(String::from(""), action));

        if self.action_history.len() > history_limit {
            self.action_history.remove(0);
        }
    }

    fn send_openai_request(&mut self, input: &str) -> Result<Action, ()> {
        let client = reqwest::blocking::Client::new();

        let body = json!({
            "model": "gpt-3.5-turbo",
            "messages": [
                {
                    "role": "system",
                    "content": self.system_prompt
                },
                {
                    "role": "user",
                    "content": input
                }
            ]
        });

        writeln!(
            self.log,
            "Sending request to OpenAI\n--------\n{}",
            serde_json::to_string_pretty(&body).unwrap()
        )
        .unwrap();
        writeln!(self.log, "--------").unwrap();

        let response = client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .body(body.to_string())
            .send()
            .expect("Failed to send request.");

        // Check if the response status is success
        if response.status().is_success() {
            let response_json: Value = response.json().expect("Failed to parse response.");
            let mut response_text = response_json["choices"][0]["message"]["content"]
                .as_str()
                .expect("Failed to parse response.");

            if response_text.starts_with("response:\n") {
                response_text = response_text.trim_start_matches("response:\n").trim();
            }

            writeln!(
                self.log,
                "Received response from OpenAI\n--------\n{:?}",
                &response_text
            )
            .unwrap();
            writeln!(self.log, "--------").unwrap();
            let action = Action::from_yaml(&response_text).expect("Failed to parse response.");
            return Ok(action);
        } else {
            panic!(
                "Failed to get a successful response: {:?}",
                response.status()
            );
        }
    }
}
