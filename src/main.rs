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
    pub mod inspectors;
    pub mod labels;
}

use std::sync::Arc;

use iced::{
    theme::{Custom, Palette},
    window::{Position, Settings},
    Color, Theme,
};
use widget::app::ResultsInspector;

use crate::game::*;

pub fn main() -> iced::Result {
    let palette = Palette {
        background: Color::BLACK,
        ..Palette::DARK
    };

    iced::application("Viewer", ResultsInspector::update, ResultsInspector::view)
        .theme(move |_| Theme::Custom(Arc::new(Custom::new(String::from("Viewer Theme"), palette))))
        .window(Settings {
            position: Position::Centered,
            ..Default::default()
        })
        .run_with(ResultsInspector::new)
}
