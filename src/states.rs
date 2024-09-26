use crossterm::event::{self, KeyCode};

enum State {
    Settings,
    Files,
    Other,
}

pub struct StateMachine {
    state: State,
}

impl StateMachine {
    fn handle_transition(&mut self, key: event::KeyEvent) {
        match key.code {
            KeyCode::Char('j') | KeyCode::Char('k') => match self.state {
                State::Settings => self.state = State::Other,
                State::Other => self.state = State::Settings,
                _ => {}
            }
            KeyCode::Char('h') | KeyCode::Char('l') => match self.state {
                State::Settings | State::Other => self.state = State::Files,
                State::Files => self.state = State::Settings,
                _ => {}
            }
            _ => {}
        }
    }

    fn handle_control(&mut self, key: event::KeyEvent) {

    }

    pub fn delegate_input(&mut self, key: event::KeyEvent) {
        match key.modifiers {
            event::KeyModifiers::SHIFT => self.handle_transition(key),
            _ => self.handle_control(key),
        }
    }

    pub fn new() -> Self {
        StateMachine { state: State::Settings }
    }
}
