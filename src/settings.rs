// Setting object
use std::collections::VecDeque;

enum SettingVal {
    Check(bool),
    Options(VecDeque<String>),
}

pub struct Setting {
    title: String,
    value: SettingVal,
}

impl Setting {
    pub fn new_check_setting(title: &str, value: bool) -> Self {
        Setting {
            title: title.to_string(),
            value: SettingVal::Check(value),
        }
    }

    pub fn new_options_setting(title: &str, options: VecDeque<String>) -> Self {
        Setting {
            title: title.to_string(),
            value: SettingVal::Options(options),
        }
    }

    pub fn current_display(&self, title_width: usize, setting_width: usize) -> String {
        let t_adj = format!("{}:", self.title);
        match &self.value {
            SettingVal::Check(v) =>
                format!("{:<title_width$} {:^setting_width$}", t_adj, if *v { "[X]" } else { "[ ]" }),
            SettingVal::Options(v) =>
                format!("{:<title_width$} ◀{:^setting_width$}▶", t_adj, v[0])
        }
    }

    pub fn cycle_setting(&mut self, forward: bool) {
        match &mut self.value {
            SettingVal::Options(ref mut settings) => {
                if forward {
                    let current_value = settings.pop_front().expect("Empty settings");
                    settings.push_back(current_value);
                } else {
                    let current_value = settings.pop_back().expect("Empty settings");
                    settings.push_front(current_value);
                }
            }
            _ => {}
        }
    }

    pub fn toggle_setting(&mut self) {
        match &mut self.value {
            SettingVal::Check(ref mut check) => {
                *check = !*check;
            },
            _ => {}
        }
    }
}
