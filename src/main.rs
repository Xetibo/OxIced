use std::env;

use crate::tools::{palette::palette, test_app::test_app};

pub mod theme;
mod tools;
mod utils;
pub mod widgets;

pub fn main() -> Result<(), iced::Error> {
    let args: Vec<String> = env::args().collect();
    let tool = args.get(1).unwrap_or(&String::from("test_app")).clone();
    match tool.as_str() {
        "palette" => palette(),
        _ => test_app(),
    }
}
