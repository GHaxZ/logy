use crossterm::style::Color;
use log::{LogStyle, LogType};

mod log;

fn main() {
    info!("This is an info!!!");
    warning!("This is a warning!!!");
    error!("This is an error!!!");
    log!(
        LogType::Custom(LogStyle {
            color: Color::Rgb {
                r: 174,
                g: 31,
                b: 255
            },
            prefix: "~ DEEZ NUTS ~ ",
            color_message: true
        }),
        "This is a custom log!!!"
    );
}
