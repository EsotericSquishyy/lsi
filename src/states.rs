use crossterm::event::{self, KeyCode};
use crate::settings::Setting;

#[derive(Clone)]
struct SettingsWindow(usize, Vec<Setting>);

#[derive(Clone)]
struct FilesWindow;

#[derive(Clone)]
struct OtherWindow;

enum State {
    Settings(SettingsWindow),
    Files(FilesWindow),
    Other(OtherWindow),
}

pub struct StateMachine {
    active: State,
    settings_window: SettingsWindow,
    files_window: FilesWindow,
    other_window: OtherWindow,
}

impl StateMachine {
    fn handle_transition(&mut self, key: event::KeyEvent) {
        match key.code {
            KeyCode::Char('j') | KeyCode::Char('k') => match self.active {
                State::Settings(ref mut settings_window) => {
                    self.settings_window = settings_window.clone();
                    self.active = State::Other(self.other_window.clone());
                }
                State::Other(other_window) => {
                    self.other_window = other_window;
                    self.active = State::Settings(self.settings_window.clone());
                }
                _ => {}
            }
            KeyCode::Char('h') | KeyCode::Char('l') => match self.active {
                State::Settings(ref mut settings_window)=> {
                    self.settings_window = settings_window.clone();
                    self.active = State::Files(self.files_window.clone());
                }
                State::Other(other_window)=> {
                    self.other_window = other_window.clone();
                    self.active = State::Files(self.files_window.clone());
                }
                State::Files(files_window) => {
                    self.files_window = files_window.clone();
                    self.active = State::Settings(self.settings_window.clone());
                }
            }
            _ => {}
        }
    }

    fn control_settings(&mut self, settings_window: &mut SettingsWindow, key: event::KeyEvent) {
        let index = &mut settings_window.0;
        let settings = &mut settings_window.1;
        match key.code {
            KeyCode::Char('j') => {
                if *index < settings.len() - 1 {
                    *index += 1;
                }
            },
            KeyCode::Char('k') => {
                if *index > 0 {
                    *index -= 1;
                }
            },
            KeyCode::Char('h') =>
                settings[*index].cycle_setting(false),
            KeyCode::Char('l') =>
                settings[*index].cycle_setting(true),
            KeyCode::Enter =>
                settings[*index].toggle_setting(),
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

    pub fn get_settings_index(&mut self) -> usize {
        match self.active {
            State::Settings(ref mut settings_window) => settings_window.0,
            _ => self.settings_window.0
        }
    }

    pub fn delegate_input(&mut self, key: event::KeyEvent) {
        match key.modifiers {
            event::KeyModifiers::SHIFT => self.handle_transition(key),
            _ => match self.active {
                State::Settings(ref mut settings_window) => self.control_settings(settings_window, key),
                State::Other(_) => self.control_other(key),
                State::Files(_) => self.control_files(key),
            }
        }
    }

    pub fn new(settings: &mut Vec<Setting>) -> Self {
        let settings_window = SettingsWindow(0, *settings);
        let files_window = FilesWindow;
        let other_window = OtherWindow;
        StateMachine {
            active: State::Settings(settings_window.clone()),
            settings_window,
            files_window,
            other_window,
        }
    }
}
