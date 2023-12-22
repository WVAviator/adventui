use std::{
    io::stdout,
    sync::{Arc, Mutex},
};

use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use dispatcher::Dispatcher;
use message::Message;

use ratatui::prelude::{CrosstermBackend, Terminal};

use ui::UI;

mod dispatcher;
mod message;
mod model;
mod ui;

fn main() -> std::io::Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut terminal =
        Terminal::new(CrosstermBackend::new(stdout())).expect("Unable to create terminal UI.");
    terminal.clear().expect("Failed to clear terminal.");

    let terminal = Arc::new(Mutex::new(terminal));

    let (app_state_tx, app_state_rx) = std::sync::mpsc::channel();
    let (ui_state_tx, ui_state_rx) = std::sync::mpsc::channel();

    let mut ui = UI::new(ui_state_rx, terminal.clone());
    let mut dispatcher = Dispatcher::new(app_state_tx, ui_state_tx);

    loop {
        match app_state_rx.recv() {
            Ok(Message::Terminate) => {
                break;
            }
            Ok(_) => {}
            Err(_) => {
                break;
            }
        }
    }

    dispatcher
        .close()
        .expect("Unable to rejoin dispatcher thread.");
    ui.close().expect("Unable to rejoin UI thread.");

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
