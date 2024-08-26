use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::MutexGuard;
use std::thread;
use std::thread::JoinHandle;
use std::{fs::OpenOptions, sync::Mutex};

use chrono::Local;
use crossterm::style::Stylize;
use once_cell::sync::Lazy;

use crate::log;
use crate::model::LogTask;
use crate::model::{LogComponent, LogMessage, LogStyle, LogType};

pub static LOG: Lazy<Mutex<Logger>> = Lazy::new(|| Mutex::new(Logger::default()));

pub struct Logger {
    pub console: bool,
    pub file: bool,
    pub threaded: bool,
    thread_handle: Option<JoinHandle<Result<(), Box<dyn Error + Send>>>>,
    thread_sender: Option<Sender<LogTask>>,
    pub output_file: &'static str,
    output_file_handle: Mutex<Option<File>>,
    pub components: Vec<LogComponent>,
    pub hooks: Vec<Box<dyn Fn(LogMessage) + Send>>,
}

impl Drop for Logger {
    fn drop(&mut self) {
        if self.threaded {
            self.set_threaded(false);
        }
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self {
            console: true,
            file: false,
            threaded: false,
            thread_handle: None,
            thread_sender: None,
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

    pub fn set_threaded(&mut self, threaded: bool) -> &mut Self {
        self.threaded = threaded;

        // Initialize the thread when threading is enabled
        if self.threaded && self.thread_handle.is_none() {
            // Create the channels for message passing
            let (tx, rx): (Sender<LogTask>, Receiver<LogTask>) = mpsc::channel();

            // Create the actual executor thread
            let thread = thread::spawn(move || -> Result<(), Box<dyn Error + Send>> {
                while let Ok(task) = rx.recv() {
                    task();
                }
                Ok(())
            });

            self.thread_sender = Some(tx);
            self.thread_handle = Some(thread);
        } else if !self.threaded && self.thread_handle.is_some() {
            self.stop_thread();
        }
        self
    }

    fn stop_thread(&mut self) {
        if let Some(handle) = self.thread_handle.take() {
            match handle.join() {
                Ok(result) => {
                    if let Err(e) = result {
                        eprintln!("Thread encountered an error: {}", e);
                    }
                }
                Err(e) => eprintln!("Failed to join log thread: {:?}", e),
            }
        }
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
        F: Fn(LogMessage) + Send + 'static,
    {
        self.hooks.push(Box::new(hook));
        self
    }

    pub fn execute_log(&mut self, log_type: LogType, message: &'static str) {
        if self.threaded {
            let task: LogTask = Box::new(move || {
                log!(log_type.clone(), message);
            });

            if let Err(e) = self.thread_sender.as_ref().unwrap().send(task) {
                eprintln!(
                    "Failed sending logging instructions to other thread, disabling threading: {}",
                    e
                );

                self.set_threaded(false);
            } else {
                return;
            }
        } else {
            self.log(log_type, message);
        }
    }

    pub fn log(&self, log_type: LogType, message: &'static str) {
        let log_message = LogMessage {
            log_type: log_type.clone(),
            log_message: message.to_string(),
            log_style: match log_type {
                LogType::Info => LogStyle::info(),
                LogType::Warning => LogStyle::warning(),
                LogType::Error => LogStyle::error(),
                LogType::Fatal => LogStyle::fatal(),
                LogType::Custom(style) => style,
            },
        };

        let log_string_console = build_log_string(&self.components, &log_message, true);
        let log_string_file = build_log_string(&self.components, &log_message, false);

        if self.file {
            self.write_to_file(&log_string_file);
        }

        if self.console {
            println!("{}", log_string_console);
        }

        self.execute_hooks(log_message);
    }

    fn write_to_file(&self, log_string_file: &str) {
        let mut file_handle = self.output_file_handle.lock().unwrap();

        if file_handle.is_none() {
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

    fn execute_hooks(&self, log_message: LogMessage) {
        for hook in self.hooks.iter() {
            hook(log_message.clone());
        }
    }
}

fn build_log_string(
    components: &Vec<LogComponent>,
    message: &LogMessage,
    with_color: bool,
) -> String {
    let mut str = String::new();

    for component in components.iter() {
        str.push_str(&get_component_str(component, &message, with_color));
    }

    str
}

fn get_component_str(
    log_component: &LogComponent,
    message: &LogMessage,
    with_color: bool,
) -> String {
    let style = &message.log_style;
    let message_str = message.log_message.to_owned();

    match log_component {
        LogComponent::Prefix => {
            if with_color {
                style.prefix.with(style.color).to_string()
            } else {
                style.prefix.to_string()
            }
        }
        LogComponent::Message => {
            if with_color && style.color_message {
                message_str.with(style.color).to_string()
            } else {
                message_str.to_string()
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
