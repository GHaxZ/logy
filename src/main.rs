use crossterm::style::Color;
use logy::logger::LOG;
use logy::model::{LogComponent, LogStyleBuilder, LogType, LoggerBuilder};
use logy::{error, fatal, info, log, warn};

fn main() {
    // Simple pre-configured logging with macros
    info!("Everything is fine ヽ(•‿•)ノ");
    warn!("This is ok (•‿•)");
    error!("This is bad (•_•)");
    fatal!("Oh shit (☉_☉)");

    // Log messages with a specified LogType
    log!(LogType::Fatal, "This is a LogType preset");

    // Log message with a custom LogType
    log!(
        LogType::Custom(
            LogStyleBuilder::new()
                .color(Color::Rgb {
                    r: 163,
                    g: 0,
                    b: 233
                })
                .prefix("*** WOW ***")
                .color_message(true)
                .build()
        ),
        "This is my own log type :D"
    );

    // Configure the global Logger
    LOG.lock().unwrap().set_file(false).set_console(true);
    info!("The configration for the global logger was just changed!");

    // Create and use your own custom Loggers
    let logger = LoggerBuilder::new()
        .console(true)
        .file(true)
        .components(vec![
            LogComponent::Prefix,
            LogComponent::String("->"),
            LogComponent::Spacer,
            LogComponent::Message,
            LogComponent::Spacer,
            LogComponent::Spacer,
            LogComponent::Spacer,
            LogComponent::Newline,
            LogComponent::Prefix,
        ])
        .build();

    logger.log(
        LogType::Info,
        "This message was logged using my own custom logger!",
    );
}
