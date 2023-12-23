use std::{
    cmp::min,
    io::{self, Stdout},
    sync::{Arc, Mutex},
};

use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{CrosstermBackend, Terminal},
    style::Color,
    text::Text,
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
};

use crossterm::{
    cursor::{EnableBlinking, MoveTo, SetCursorStyle, Show},
    execute,
};

use crate::model::game_state::GameState;

pub struct GameRenderer {
    terminal: Arc<Mutex<Terminal<CrosstermBackend<Stdout>>>>,
}

impl GameRenderer {
    pub fn new(terminal: Arc<Mutex<Terminal<CrosstermBackend<Stdout>>>>) -> Self {
        GameRenderer { terminal }
    }

    pub fn render(&self, state: GameState) {
        let mut terminal = self
            .terminal
            .lock()
            .expect("Unable to get lock on terminal for UI rendering.");
        let margin = 1;
        terminal
            .draw(|frame| {
                let size = frame.size();
                let inventory_width = get_inventory_width(state.get_inventory());
                let horizontal_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .margin(margin)
                    .constraints(
                        [Constraint::Min(10), Constraint::Length(inventory_width)].as_ref(),
                    )
                    .split(size);

                let description = Paragraph::new(state.get_scene_desc())
                    .style(ratatui::style::Style::default().fg(Color::Yellow))
                    .wrap(Wrap::default())
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title(state.get_scene_title()),
                    );
                let desc_height = description.line_count(horizontal_chunks[0].width - 2) as u16;

                let vertical_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        [
                            Constraint::Length(desc_height + 2),
                            Constraint::Min(5),
                            Constraint::Length(3),
                        ]
                        .as_ref(),
                    )
                    .split(horizontal_chunks[0]);
                frame.render_widget(description, vertical_chunks[0]);

                let history_items = paragraph_list_to_lines(
                    state.get_scene_history(),
                    vertical_chunks[1].width - 2,
                );

                let max_history_items =
                    min(vertical_chunks[1].height - 2, history_items.len() as u16) as usize;

                let skip_lines = std::cmp::max(
                    0,
                    history_items.len() as isize
                        - state.get_scroll_position() as isize
                        - max_history_items as isize,
                ) as usize;

                let history_items: Vec<ListItem> = history_items
                    .iter()
                    .skip(skip_lines)
                    .take(max_history_items)
                    .map(|item| ListItem::new(Text::raw(item)))
                    .collect();

                frame.render_widget(
                    List::new(history_items).block(Block::default().borders(Borders::ALL)),
                    vertical_chunks[1],
                );
                frame.render_widget(
                    Paragraph::new(state.get_user_entry())
                        .style(ratatui::style::Style::default().fg(Color::White))
                        .block(Block::default().borders(Borders::ALL).title("Input")),
                    vertical_chunks[2],
                );

                let inventory_items: Vec<ListItem> = state
                    .get_inventory()
                    .iter()
                    .map(|item| {
                        if item.len() > 20 {
                            return format!("{}..", &item[..18]);
                        }
                        item.to_string()
                    })
                    .map(|item| ListItem::new(Text::raw(item)))
                    .collect();

                frame.render_widget(
                    List::new(inventory_items)
                        .block(Block::default().borders(Borders::ALL).title("Inventory")),
                    horizontal_chunks[1],
                );
            })
            .expect("Failed to draw game frame.");

        let user_input_y = terminal
            .size()
            .expect("Unable to get terminal size.")
            .height
            - 2
            - margin;

        // Moves the position and display style of the terminal cursor to match the input.
        execute!(
            io::stdout(),
            MoveTo(
                margin + 1 + state.get_user_entry().len() as u16,
                user_input_y
            ),
            EnableBlinking,
            Show,
            SetCursorStyle::BlinkingBar
        )
        .expect("Unable to reposition cursor for user input.")
    }
}

fn paragraph_list_to_lines(paragraphs: &Vec<String>, width: u16) -> Vec<String> {
    let mut lines = Vec::new();
    for paragraph in paragraphs {
        let mut paragraph_lines = Vec::new();
        let mut line = String::new();
        for word in paragraph.split_whitespace() {
            if line.len() + word.len() + 1 > width as usize {
                paragraph_lines.push(line.clone());
                line.clear();
            }
            line += word;
            line += " ";
        }
        paragraph_lines.push(line);
        lines.extend(paragraph_lines);
        lines.push(String::new());
    }
    lines
}

fn get_inventory_width(inventory: &[String]) -> u16 {
    std::cmp::min(
        std::cmp::max(
            12,
            inventory.iter().map(|item| item.len()).max().unwrap_or(0) + 2,
        ) as u16,
        20,
    )
}
