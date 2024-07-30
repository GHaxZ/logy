use crossterm::style::Color;
use logy::logging::logger;
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

    // Add a hook which will be called every time something is logged
    logger().add_hook(|log| println!("This is a hook for a log of type '{:?}'!!!", log.log_type));

    // Configure the global Logger
    logger().set_console(true).set_file(true);
    info!("The configration for the global logger was just changed!");

    // Create and use your own custom Loggers
    let logger = LoggerBuilder::new()
        .console(true)
        .file(true)
        .output_file("test.log")
        .components(vec![
            LogComponent::Prefix,
            LogComponent::Spacer,
            LogComponent::String("->"),
            LogComponent::Spacer,
            LogComponent::Message,
            LogComponent::Spacer,
            LogComponent::Spacer,
            LogComponent::Spacer,
            LogComponent::Newline,
            LogComponent::Prefix,
        ])
        .add_hook(|log| println!("A log of type '{:?}' occurred!", log.log_type))
        .build();

    logger.log(
        LogType::Warning,
        "This message was logged using my own custom logger!",
    );
}
