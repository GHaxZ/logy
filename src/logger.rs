use std::{sync::Mutex, time::SystemTime};

use crossterm::style::Stylize;
use once_cell::sync::Lazy;

use crate::model::{LogComponent, LogStyle, LogType};

pub static LOG: Lazy<Mutex<Logger>> = Lazy::new(|| Mutex::new(Logger::default()));

pub struct Logger {
    pub console: bool,
    pub file: bool,
    pub components: Vec<LogComponent>,
}

impl Default for Logger {
    fn default() -> Self {
        Self {
            console: true,
            file: false,
            components: vec![
                LogComponent::Time,
                LogComponent::Spacer,
                LogComponent::Prefix,
                LogComponent::Spacer,
                LogComponent::Message,
            ],
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

    pub fn set_components(&mut self, components: Vec<LogComponent>) -> &mut Self {
        self.components = components;

        self
    }

    pub fn log(&self, log_type: LogType, message: &str) {
        println!(
            "{}",
            build_log_string(self.components.clone(), log_type, message)
        );
    }
}

fn build_log_string(components: Vec<LogComponent>, log_type: LogType, message: &str) -> String {
    let style = match log_type {
        LogType::Info => LogStyle::info(),
        LogType::Warning => LogStyle::warning(),
        LogType::Error => LogStyle::error(),
        LogType::Fatal => LogStyle::fatal(),
        LogType::Custom(style) => style,
    };

    let mut str = String::new();

    for component in components.into_iter() {
        str.push_str(get_component_str(component, style, message).as_str());
    }

    str
}

fn get_component_str(log_component: LogComponent, log_style: LogStyle, message: &str) -> String {
    match log_component {
        LogComponent::Prefix => log_style.prefix.with(log_style.color).to_string(),
        LogComponent::Message => {
            if log_style.color_message {
                message.with(log_style.color).to_string()
            } else {
                message.to_string()
            }
        }
        LogComponent::Time => format!("{:?}", SystemTime::now()),
        LogComponent::Spacer => " ".to_string(),
        LogComponent::Newline => "\n".to_string(),
        LogComponent::String(s) => s.to_string(),
    }
}
