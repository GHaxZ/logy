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
    pub color: Color,
    pub prefix: &'static str,
    pub color_message: bool,
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
        LogType::Fatal => LogStyle {
            color: Color::DarkRed,
            prefix: "[FATAL] ",
            color_message: true,
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
