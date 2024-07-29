use std::fs::File;
use std::io::Write;
use std::sync::MutexGuard;
use std::{fs::OpenOptions, sync::Mutex};

use chrono::Local;
use crossterm::style::Stylize;
use once_cell::sync::Lazy;

use crate::model::{LogComponent, LogStyle, LogType};

static LOG: Lazy<Mutex<Logger>> = Lazy::new(|| Mutex::new(Logger::default()));

pub struct Logger {
    pub console: bool,
    pub file: bool,
    pub output_file: &'static str,
    output_file_handle: Mutex<Option<File>>,
    pub components: Vec<LogComponent>,
    pub hooks: Vec<Box<dyn Fn(LogType) + Send>>,
}

impl Default for Logger {
    fn default() -> Self {
        Self {
            console: true,
            file: false,
            output_file: "debug.log",
            output_file_handle: Mutex::new(None),
            components: vec![
                LogComponent::Time,
                LogComponent::Spacer,
                LogComponent::Prefix,
                LogComponent::Spacer,
                LogComponent::Message,
            ],
            hooks: vec![],
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
        if !self.file {
            let mut file_handle = self.output_file_handle.lock().unwrap();
            *file_handle = None;
        }
        self
    }

    pub fn set_output_file(&mut self, output_file: &'static str) -> &mut Self {
        self.output_file = output_file;
        self
    }

    pub fn set_components(&mut self, components: Vec<LogComponent>) -> &mut Self {
        self.components = components;
        self
    }

    pub fn add_hook<F>(&mut self, hook: F) -> &mut Self
    where
        F: Fn(LogType) + Send + 'static,
    {
        self.hooks.push(Box::new(hook));
        self
    }

    pub fn log(&self, log_type: LogType, message: &str) {
        let log_string_console = build_log_string(&self.components, &log_type, message, true);
        let log_string_file = build_log_string(&self.components, &log_type, message, false);

        if self.file {
            let mut file_handle = self.output_file_handle.lock().unwrap();

            if file_handle.is_none() {
                // Create the file if it doesn't exist
                let file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .create(true)
                    .open(self.output_file);

                match file {
                    Ok(f) => *file_handle = Some(f),
                    Err(e) => eprintln!("Failed opening log file: {}", e),
                }
            }

            if let Some(ref mut log_file) = *file_handle {
                if let Err(e) = writeln!(log_file, "{}", log_string_file) {
                    eprintln!("Failed to write to log file: {}", e);
                }
            }
        }

        if self.console {
            println!("{}", log_string_console);
        }

        for hook in self.hooks.iter() {
            hook(log_type.clone());
        }
    }
}

fn build_log_string(
    components: &Vec<LogComponent>,
    log_type: &LogType,
    message: &str,
    with_color: bool,
) -> String {
    let style = match log_type {
        LogType::Info => LogStyle::info(),
        LogType::Warning => LogStyle::warning(),
        LogType::Error => LogStyle::error(),
        LogType::Fatal => LogStyle::fatal(),
        LogType::Custom(style) => style.clone(),
    };

    let mut str = String::new();

    for component in components.iter() {
        str.push_str(&get_component_str(component, &style, message, with_color));
    }

    str
}

fn get_component_str(
    log_component: &LogComponent,
    log_style: &LogStyle,
    message: &str,
    with_color: bool,
) -> String {
    match log_component {
        LogComponent::Prefix => {
            if with_color {
                log_style.prefix.with(log_style.color).to_string()
            } else {
                log_style.prefix.to_string()
            }
        }
        LogComponent::Message => {
            if with_color && log_style.color_message {
                message.with(log_style.color).to_string()
            } else {
                message.to_string()
            }
        }
        LogComponent::Time => Local::now().format("%H:%M:%S").to_string(),
        LogComponent::Spacer => " ".to_string(),
        LogComponent::Newline => "\n".to_string(),
        LogComponent::String(s) => s.to_string(),
    }
}

pub fn logger() -> MutexGuard<'static, Logger> {
    LOG.lock().unwrap()
}
