use crossterm::style::Color;
use logy::logging::{LogStyle, LogStyleBuilder, LogType};
use logy::{error, fatal, info, log, warning};

fn main() {
    info!("Everything is fine ヽ(•‿•)ノ");
    warning!("This is ok (•‿•)");
    error!("This is bad (•_•)");
    fatal!("Oh shit (☉_☉)");
    log!(
        LogType::Custom(
            LogStyleBuilder::new()
                .color(Color::Rgb {
                    r: 174,
                    g: 31,
                    b: 255
                })
                .prefix("~ WOW ~ ")
                .color_message(true)
                .build()
        ),
        "This is special ~(˘▾˘~)"
    );
    log!(
        LogType::Custom(
            LogStyleBuilder::from(LogStyle::warning())
                .color_message(true)
                .build()
        ),
        "Another looog"
    )
}
