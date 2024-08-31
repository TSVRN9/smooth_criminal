#![allow(dead_code)]

pub mod colors;
pub mod game;
pub mod strategies {
    pub mod classic;
    pub mod continuous;
    pub mod tsvrn9;
    pub mod utils;
}
pub mod widget {
    pub mod app;
    pub mod grid;
    pub mod labels;
}

use iced::{window::{Position, Settings}, Theme};
use widget::app::ResultsInspector;

use crate::game::*;

pub fn main() -> iced::Result {
    iced::application("Viewer", ResultsInspector::update, ResultsInspector::view)
        .theme(|_| Theme::Dark)
        .window(Settings {
            position: Position::Centered,
            ..Default::default()
        })
        .run_with(ResultsInspector::new)
}
