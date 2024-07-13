use crossterm::style::{Color, Stylize};

pub enum LogType {
    Info,
    Warning,
    Error,
    Fatal,
    Custom(LogStyle),
}

#[derive(Clone)]
pub struct LogStyle {
    color: Color,
    prefix: &'static str,
    color_message: bool,
}

impl LogStyle {
    pub fn default() -> Self {
        Self {
            color: Color::White,
            prefix: "[LOG] ",
            color_message: false,
        }
    }

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

pub fn log_message(log_type: LogType, message: &'static str) {
    println!("{}", build_log_string(log_type, message));
}

fn build_log_string(log_type: LogType, message: &'static str) -> String {
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

    return str;
}

#[macro_export]
macro_rules! info {
    ($message:expr) => {
        $crate::logging::log_message($crate::logging::LogType::Info, $message);
    };
}

#[macro_export]
macro_rules! warning {
    ($message:expr) => {
        $crate::logging::log_message($crate::logging::LogType::Warning, $message);
    };
}

#[macro_export]
macro_rules! error {
    ($message:expr) => {
        $crate::logging::log_message($crate::logging::LogType::Error, $message);
    };
}

#[macro_export]
macro_rules! fatal {
    ($message:expr) => {
        $crate::logging::log_message($crate::logging::LogType::Fatal, $message);
    };
}

#[macro_export]
macro_rules! log {
    ($style:expr, $message:expr) => {
        $crate::logging::log_message($style, $message);
    };
}
