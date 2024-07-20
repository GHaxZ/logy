use crossterm::style::Color;

use crate::logger::Logger;

pub enum LogType {
    Info,
    Warning,
    Error,
    Fatal,
    Custom(LogStyle),
}

#[derive(Clone)]
pub struct LogStyle {
    pub color: Color,
    pub prefix: &'static str,
    pub color_message: bool,
}

impl Default for LogStyle {
    fn default() -> Self {
        Self {
            color: Color::White,
            prefix: "[LOG] ",
            color_message: false,
        }
    }
}

impl LogStyle {
    pub fn info() -> Self {
        Self {
            color: Color::White,
            prefix: "[INFO] ",
            color_message: false,
        }
    }

    pub fn warning() -> Self {
        Self {
            color: Color::Yellow,
            prefix: "[WARNING] ",
            color_message: false,
        }
    }

    pub fn error() -> Self {
        Self {
            color: Color::Red,
            prefix: "[ERROR] ",
            color_message: false,
        }
    }

    pub fn fatal() -> Self {
        Self {
            color: Color::DarkRed,
            prefix: "[FATAL] ",
            color_message: true,
        }
    }
}

pub struct LoggerBuilder {
    logger: Logger,
}

impl LoggerBuilder {
    pub fn new() -> Self {
        Self {
            logger: Logger::default(),
        }
    }

    pub fn console(mut self, console: bool) -> Self {
        self.logger.console = console;

        self
    }

    pub fn file(mut self, file: bool) -> Self {
        self.logger.file = file;

        self
    }

    pub fn build(self) -> Logger {
        self.logger
    }
}

pub struct LogStyleBuilder {
    style: LogStyle,
}

impl LogStyleBuilder {
    pub fn new() -> Self {
        Self {
            style: LogStyle::default(),
        }
    }

    pub fn from(style: LogStyle) -> Self {
        Self { style }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.style.color = color;

        self
    }

    pub fn prefix(mut self, prefix: &'static str) -> Self {
        self.style.prefix = prefix;

        self
    }

    pub fn color_message(mut self, color_message: bool) -> Self {
        self.style.color_message = color_message;

        self
    }

    pub fn build(self) -> LogStyle {
        self.style
    }
}
