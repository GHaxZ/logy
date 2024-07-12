use crossterm::style::{Color, Stylize};

pub enum LogType {
    Info,
    Warning,
    Error,
    Custom(LogStyle),
}

#[derive(Clone)]
pub struct LogStyle {
    pub color: Color,
    pub prefix: &'static str,
    pub color_message: bool,
}

impl LogType {
    fn new_info() -> Self {
        Self::Info
    }

    fn new_warning() -> Self {
        Self::Warning
    }

    fn new_error() -> Self {
        Self::Error
    }

    pub fn new_custom(style: LogStyle) -> Self {
        Self::Custom(style)
    }
}

pub fn log_message(log_type: LogType, message: &'static str) {
    let style = match log_type {
        LogType::Info => LogStyle {
            color: Color::White,
            prefix: "[INFO] ",
            color_message: false,
        },
        LogType::Warning => LogStyle {
            color: Color::Yellow,
            prefix: "[WARNING] ",
            color_message: false,
        },
        LogType::Error => LogStyle {
            color: Color::Red,
            prefix: "[ERROR] ",
            color_message: false,
        },
        LogType::Custom(style) => style,
    };

    let mut str = String::new();
    str.push_str(&style.prefix.with(style.color).to_string());
    if style.color_message {
        str.push_str(&message.with(style.color).to_string());
    } else {
        str.push_str(message);
    }

    println!("{}", str);
}

#[macro_export]
macro_rules! info {
    ($message:expr) => {
        $crate::log::log_message($crate::log::LogType::Info, $message);
    };
}

#[macro_export]
macro_rules! warning {
    ($message:expr) => {
        $crate::log::log_message($crate::log::LogType::Warning, $message);
    };
}

#[macro_export]
macro_rules! error {
    ($message:expr) => {
        $crate::log::log_message($crate::log::LogType::Error, $message);
    };
}

#[macro_export]
macro_rules! log {
    ($style:expr, $message:expr) => {
        $crate::log::log_message($style, $message);
    };
}
