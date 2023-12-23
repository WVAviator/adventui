# HarmoniCLI

HarmoniCLI is a simple CLI text adventure game platform that uses modern LLM capabilities to generate completely custom adventures. There are no limits to the commands players can make in regards to investigating locations, collecting objects, having conversations with non-player characters, and moving between areas. 

## Technical Details

HarmoniCLI is built in Rust, and is a Ratatui / Crossterm CLI application with completely decoupled application and UI state. Events handling and UI rendering are both managed in their own threads, and state updates are communicated one-way to the UI using mpsc message passing.

Currently HarmoniCLI is set up to use the OpenAI API to drive the gameplay. However, there is no reason that other LLMs could not be substituted in the future. There are certainly improvements to be made regarding the prompts and context used in the game.

HarmoniCLI is under active development without a release yet, however it is currently (barely) functioning. If you'd like to contribute - then go ahead and make an issue, fork, and clone the project. 
