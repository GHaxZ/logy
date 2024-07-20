#[macro_export]
macro_rules! log {
    ($type:expr, $msg:expr) => {
        $crate::logger::LOG.lock().unwrap().log($type, $msg);
    };
}

#[macro_export]
macro_rules! info {
    ($msg:expr) => {
        $crate::log!($crate::model::LogType::Info, $msg);
    };
}

#[macro_export]
macro_rules! warn {
    ($msg:expr) => {
        $crate::log!($crate::model::LogType::Warning, $msg);
    };
}

#[macro_export]
macro_rules! error {
    ($msg:expr) => {
        $crate::log!($crate::model::LogType::Error, $msg);
    };
}

#[macro_export]
macro_rules! fatal {
    ($msg:expr) => {
        $crate::log!($crate::model::LogType::Fatal, $msg);
    };
}
