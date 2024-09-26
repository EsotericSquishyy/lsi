// Main executable

pub mod files;
pub mod settings;
pub mod states;

use crate::settings::Setting;
use crate::states::StateMachine;
use std::collections::VecDeque;

use crossterm::{
    event::{self, KeyCode, KeyEvent},
    execute,
    terminal::{self, ClearType},
    ExecutableCommand,
};
use std::{error::Error, io::{self, Write}, time::Duration};
use tui::{
    style::{Color, Style},
    backend::CrosstermBackend,
    layout::{Layout, Rect, Constraint, Direction},
    widgets::{Block, Borders, List, ListItem, ListState, Tabs},
    Terminal
};

fn main() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(terminal::Clear(ClearType::All))?;
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend)?;

    let mut settings = vec![
        Setting::new_options_setting("Visual Mode", VecDeque::from(["List".to_string(), "Tree".to_string()])),
        Setting::new_options_setting("Visibility", VecDeque::from(["Normal".to_string(), "Most".to_string(), "All".to_string()])),
        Setting::new_options_setting("Color Mode", VecDeque::from(["Default".to_string(), "Normal".to_string(), "High Contrast".to_string()])),
        Setting::new_options_setting("File Size", VecDeque::from(["None".to_string(), "Bytes".to_string(), "Human Readable".to_string()])),
        Setting::new_check_setting("Permissions", false),
        Setting::new_check_setting("Date/Time", false),
        Setting::new_check_setting("User", false),
        Setting::new_check_setting("Group", false),
        Setting::new_options_setting("Depth", VecDeque::from(["Infinite".to_string(), "1".to_string(), "2".to_string(), "3".to_string()]))
    ];
    let mut selected_index: usize = 0;

    // Find State
    let mut list_state = ListState::default();
    list_state.select(Some(selected_index));

    let mut state = StateMachine::new(selected_index, settings.len());

    loop {

        // Draw UI
        term.draw(|f| {
            let size = f.size();
            let layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Percentage(30),
                        Constraint::Percentage(70),
                    ]
                    .as_ref()
                )
                .split(size);

            let left_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(50),
                        Constraint::Percentage(50),
                    ]
                    .as_ref()
                )
                .split(layout[0]);

            let options_block   = Block::default().title("Options").borders(Borders::ALL);
            let other_block     = Block::default().title("Other").borders(Borders::ALL);
            let files_block     = Block::default().title("Files").borders(Borders::ALL);

            let titles: Vec<ListItem> = settings
                .iter()
                .map(|s| ListItem::new(s.current_display(14, 17)))
                .collect();
            let list = List::new(titles)
                .block(options_block)
                .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

            let titles = ["Tab1", "Tab2", "Tab3", "Tab4"].iter().cloned().map(tui::text::Spans::from).collect();
            let tabs = Tabs::new(titles)
                .block(files_block)
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().fg(Color::Yellow))
                .divider(tui::symbols::DOT);

            f.render_stateful_widget(list, left_chunks[0], &mut list_state);
            f.render_widget(tabs, layout[1]);
            f.render_widget(other_block, left_chunks[1]);
        })?;

        // Handle user input
        if event::poll(Duration::from_millis(200))? {
            if let event::Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc | KeyCode::Char('q') => break,
                    _ => state.delegate_input(key),
                }
            }
            list_state.select(Some(selected_index));
        }
    }

    // Clean up
    term.clear()?;
    terminal::disable_raw_mode()?;

    let mut final_stdout = io::stdout(); // Create a new variable to avoid the borrow checker issue
    final_stdout.execute(crossterm::cursor::MoveTo(0, 0))?; // Move cursor to top left

    // Closure message
    // println!("Exited TUI. Press Enter to continue...");
    // let _ = std::io::stdin().read_line(&mut String::new());

    Ok(())
}

