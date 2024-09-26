use crossterm::event::{self, KeyCode};

struct SettingsWindow(usize, usize);

struct FilesWindow;

struct OtherWindow;

enum State {
    Settings(SettingsWindow),
    Files(FilesWindow),
    Other(OtherWindow),
}

pub struct StateMachine {
    active: State,
    settings: State,
    files: State,
    other: State,
}

impl StateMachine {
    fn handle_transition(&mut self, key: event::KeyEvent) {
        match key.code {
            KeyCode::Char('j') | KeyCode::Char('k') => match self.active {
                State::Settings(_) => {
                    self.settings = self.active;
                    self.active = self.other;
                }
                State::Other(_) => {
                    self.other = self.active;
                    self.active = self.settings;
                }
                _ => {}
            }
            KeyCode::Char('h') | KeyCode::Char('l') => match self.active {
                State::Settings(_)=> {
                    self.settings = self.active;
                    self.active = self.files;
                }
                State::Other(_)=> {
                    self.other = self.active;
                    self.active = self.files;
                }
                State::Files(_) => {
                    self.files = self.active;
                    self.active = self.settings;
                }
            }
            _ => {}
        }
    }

    fn control_settings(&mut self, key: event::KeyEvent) {
        match key.code {
            KeyCode::Char('j') => {
                if selected_index < settings.len() - 1 {
                    selected_index += 1;
                }
            },
            KeyCode::Char('k') => {
                if selected_index > 0 {
                    selected_index -= 1;
                }
            },
            KeyCode::Char('h') =>
                settings[selected_index].cycle_setting(false),
            KeyCode::Char('l') =>
                settings[selected_index].cycle_setting(true),
            KeyCode::Enter =>
                settings[selected_index].toggle_setting()
            _ => {}
        }
    }

    fn control_other(&mut self, key: event::KeyEvent) {
        match key.code {
            _ => {}
        }
    }

    fn control_files(&mut self, key: event::KeyEvent) {
        match key.code {
            _ => {}
        }
    }

    pub fn delegate_input(&mut self, key: event::KeyEvent) {
        match key.modifiers {
            event::KeyModifiers::SHIFT => self.handle_transition(key),
            _ => match self.active {
                State::Settings(_) => self.control_settings(key),
                State::Other(_) => self.control_other(key),
                State::Files(_) => self.control_files(key),
                _ => {}
            }
        }
    }

    pub fn new(index: usize, wrap: usize) -> Self {
        let settings = State::Settings(SettingsWindow(index, wrap));
        let files = State::Files(FilesWindow);
        let other = State::Other(OtherWindow);
        StateMachine {
            active: settings,
            settings,
            files,
            other,
        }
    }
}
