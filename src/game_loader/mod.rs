// The game loader is responsible for loading responses to each user input and logically determine what actions should result

use std::fs::{self, File, OpenOptions};
use std::io::Write;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{action::Action, model::game_state::GameState};

pub struct GameLoader {
    api_key: String,
    action_history: Vec<ActionHistoryItem>,
    system_prompt: String,
    log: File,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Context {
    inventory: Vec<String>,
    history: Vec<ActionHistoryItem>,
    input: String,
}

impl Context {
    pub fn new(inventory: Vec<String>, history: Vec<ActionHistoryItem>, input: String) -> Self {
        Context {
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
            action_history: Vec::new(),
            system_prompt,
            log,
        }
    }

    pub fn process_input(&mut self, input: &str, state: &GameState) -> Action {
        let context = Context::new(
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
