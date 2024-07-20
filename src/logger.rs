use std::sync::Mutex;

use crossterm::style::Stylize;
use once_cell::sync::Lazy;

use crate::model::{LogStyle, LogType};

pub static LOG: Lazy<Mutex<Logger>> = Lazy::new(|| Mutex::new(Logger::default()));

pub struct Logger {
    pub console: bool,
    pub file: bool,
}

impl Default for Logger {
    fn default() -> Self {
        Self {
            console: true,
            file: false,
        }
    }
}

impl Logger {
    pub fn set_console(&mut self, console: bool) -> &mut Self {
        self.console = console;

        self
    }

    pub fn set_file(&mut self, file: bool) -> &mut Self {
        self.file = file;

        self
    }

    pub fn log(&self, log_type: LogType, message: &str) {
        println!("{}", build_log_string(log_type, message));
    }
}

fn build_log_string(log_type: LogType, message: &str) -> String {
    let style = match log_type {
        LogType::Info => LogStyle::info(),
        LogType::Warning => LogStyle::warning(),
        LogType::Error => LogStyle::error(),
        LogType::Fatal => LogStyle::fatal(),
        LogType::Custom(style) => style,
    };

    let mut str = String::new();

    str.push_str(&style.prefix.with(style.color).to_string());

    if style.color_message {
        str.push_str(&message.with(style.color).to_string());
    } else {
        str.push_str(message);
    }

    str
}
