use std::{cell::Cell, default};

use iced::{
    widget::{button::{self, Style}, Space},
    Background, Color, Element, Length,
};

use crate::MatchupResult;

#[derive(Debug, Clone)]
pub enum Message {
    Select(MatchupResult),
}

#[derive(Default, Debug, Clone)]
pub enum Status {
    #[default]
    Idle,
    Focused(MatchupResult),
}

#[derive(Default, Debug, Clone)]
pub struct Grid {
    matchup_results: Vec<MatchupResult>,
    status: Status,
}

impl Grid {
    pub fn update(&mut self, message: Message) {}

    pub fn view(&self) -> Element<Message> {}
}

#[derive(Default, Debug, Clone)]
struct GridCell {
    is_selected: bool,
}

#[derive(Debug, Clone)]
enum CellMessage {
    Pressed,
}

impl GridCell {
    pub fn update(&mut self, message: CellMessage) {}

    pub fn view(&self) -> Element<CellMessage> {
        button(Space::new(Length::Fill, Length::Fill))
            .on_press(CellMessage::Pressed)
            .style(|theme, status| {
                Style::default().with_background(Background::Color(Color::BLACK))
            })
            .into()
    }
}
