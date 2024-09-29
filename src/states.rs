use crossterm::event::{self, KeyCode};
use crate::settings::Setting;

struct SettingsWindow(usize, Vec<Setting>);

struct FilesWindow;

struct OtherWindow;

#[derive(Clone, PartialEq)]
pub enum State {
    Settings,
    Files,
    Other,
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
        self.active = match key.code {
            KeyCode::Char('J') | KeyCode::Char('K') => match self.active {
                State::Settings => State::Other,
                State::Other => State::Settings,
                State::Files => State::Files
            }
            KeyCode::Char('H') | KeyCode::Char('L') => match self.active {
                State::Settings => State::Files,
                State::Other => State::Files,
                State::Files => State::Settings
            }
            _ => self.active.clone()
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
        self.settings_window.0
    }

    pub fn get_settings(& self) -> &Vec<Setting> {
        &self.settings_window.1
    }

    pub fn get_state(& self) -> State {
        self.active.clone()
    }

    pub fn delegate_input(&mut self, key: event::KeyEvent) {
        match key.modifiers {
            event::KeyModifiers::SHIFT =>
                self.handle_transition(key),
            _ => match self.active {
                State::Settings => Self::control_settings(&mut self.settings_window, key),
                State::Other => Self::control_other(key),
                State::Files => Self::control_files(key),
            }
        }
    }

    pub fn new(settings: &mut Vec<Setting>) -> Self {
        let settings_window = SettingsWindow(0, settings.clone());
        let files_window = FilesWindow;
        let other_window = OtherWindow;
        StateMachine {
            active: State::Settings,
            settings_window,
            files_window,
            other_window,
        }
    }
}
