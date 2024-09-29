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
    /*----- Private -----*/

    fn handle_transition(&mut self, key: event::KeyEvent) {
        match key.code {
            KeyCode::Char('J') | KeyCode::Char('K') => match self.active {
                State::Settings(ref settings_window) => {
                    self.settings_window = settings_window.clone();
                    self.active = State::Other(self.other_window.clone());
                }
                State::Other(ref other_window) => {
                    self.other_window = other_window.clone();
                    self.active = State::Settings(self.settings_window.clone());
                }
                _ => {}
            }
            KeyCode::Char('H') | KeyCode::Char('L') => match self.active {
                State::Settings(ref settings_window) => {
                    self.settings_window = settings_window.clone();
                    self.active = State::Files(self.files_window.clone());
                }
                State::Other(ref other_window) => {
                    self.other_window = other_window.clone();
                    self.active = State::Files(self.files_window.clone());
                }
                State::Files(ref files_window) => {
                    self.files_window = files_window.clone();
                    self.active = State::Settings(self.settings_window.clone());
                }
            }
            _ => {}
        }
    }

    fn control_settings(settings_window: &mut SettingsWindow, key: event::KeyEvent) {
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
            KeyCode::Char('h') => {
                settings[*index].cycle_setting(false);
            },
            KeyCode::Char('l') => {
                settings[*index].cycle_setting(true);},
            KeyCode::Enter => {
                settings[*index].toggle_setting();},
            _ => {}
        }
    }

    fn control_other(key: event::KeyEvent) {
        match key.code {
            _ => {}
        }
    }

    fn control_files(key: event::KeyEvent) {
        match key.code {
            _ => {}
        }
    }



    /*----- Public -----*/

    pub fn get_settings_index(& self) -> usize {
        match self.active {
            State::Settings(ref settings_window) => settings_window.0,
            _ => self.settings_window.0
        }
    }

    pub fn get_settings(& self) -> &Vec<Setting> {
        match self.active {
            State::Settings(ref settings_window) => &settings_window.1,
            _ => &self.settings_window.1
        }
    }

    // Debug
    pub fn print_state(&mut self) {
        match self.active {
            State::Settings(_) => println!("Settings"),
            State::Other(_) => println!("Other"),
            State::Files(_) => println!("Files"),
        }
    }

    pub fn delegate_input(&mut self, key: event::KeyEvent) {
        match key.modifiers {
            event::KeyModifiers::SHIFT =>
                self.handle_transition(key),
            _ => match self.active {
                State::Settings(ref mut settings_window) => Self::control_settings(settings_window, key),
                State::Other(_) => Self::control_other(key),
                State::Files(_) => Self::control_files(key),
            }
        }
    }

    pub fn new(settings: &mut Vec<Setting>) -> Self {
        let settings_window = SettingsWindow(0, settings.clone());
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
