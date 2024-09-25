// Main executable

use crossterm::{
    event::{self, KeyCode, KeyEvent},
    execute,
    terminal::{self, ClearType},
    ExecutableCommand,
};
use std::{error::Error, io::{self, Write}, time::Duration};
use tui::{
    backend::CrosstermBackend,
    layout::{Rect, Constraint, Direction},
    widgets::{Block, Borders, List, ListItem, ListState},
    Terminal
};

fn main() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(terminal::Clear(ClearType::All))?;
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend)?;

    let mut options = vec!["Option 1", "Option 2", "Option 3", "Option 4"];
    let mut selected_index = 0;

    // Find State
    let mut list_state = ListState::default();
    list_state.select(Some(selected_index));

    loop {

        // Draw UI
        term.draw(|f| {
            let size = f.size();
            let block = Block::default().title("Options").borders(Borders::ALL);
            f.render_widget(block, size);

            let items: Vec<ListItem> = options.iter().map(|o| ListItem::new(*o)).collect();
            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("Select an option"))
                .highlight_style(tui::style::Style::default()
                    .bg(tui::style::Color::Blue)
                    .fg(tui::style::Color::White)
                );
            f.render_stateful_widget(list, size, &mut list_state);
        })?;

        // Handle user input
        if event::poll(Duration::from_millis(200))? {
            if let event::Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Esc => break, // Exit on Escape
                    KeyCode::Char('h') => {
                        if selected_index > 0 {
                            selected_index -= 1;
                        }
                    }
                    KeyCode::Char('j') => {
                        if selected_index < options.len() - 1 {
                            selected_index += 1;
                        }
                    }
                    KeyCode::Char('k') => {
                        if selected_index > 0 {
                            selected_index -= 1;
                        }
                    }
                    KeyCode::Char('l') => {
                        if selected_index < options.len() - 1 {
                            selected_index += 1;
                        }
                    }
                    KeyCode::Enter => {
                        println!("Selected: {}", options[selected_index]);
                    }
                    _ => {}
                }
            }
            list_state.select(Some(selected_index));
        }
    }

    // Clean up
    term.clear()?;
    terminal::disable_raw_mode()?;
    Ok(())
}

