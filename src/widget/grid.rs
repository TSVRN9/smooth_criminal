use std::{cell::Cell, default};

use iced::{
    widget::{button, column, row, Space},
    Background, Color, Element, Length,
};

use crate::MatchupResult;

#[derive(Debug, Clone)]
pub enum Message<'a> {
    Select(&'a MatchupResult),
}

#[derive(Default, Debug, Clone)]
pub enum Status {
    #[default]
    Idle,
    Focused(MatchupResult),
}

#[derive(Debug, Clone)]
pub struct Grid<'a> {
    matchup_results: &'a Vec<MatchupResult>,
    grid_cells: Vec<GridCell<'a>>,
    status: Status,
}

impl Grid<'_> {
    pub fn new<'a>(matchup_results: &'a Vec<MatchupResult>) -> Grid<'a> {
        let grid_cells = matchup_results.iter().map(GridCell::new).collect();
        Grid {
            matchup_results,
            grid_cells,
            status: Status::Idle,
        }
    }

    pub fn update(&mut self, message: Message) {}

    pub fn view(&self) -> Element<Message> {
        let n = (self.grid_cells.len() as f64).sqrt() as usize;

        let rows = self.grid_cells.chunks(n).map(|r| {
            row(r
                .iter()
                .map(|e| e.view().map(|_| Message::Select(e.matchup_result))))
            .into()
        });

        column(rows).into()
    }
}

#[derive(Debug, Clone)]
struct GridCell<'a> {
    is_selected: bool,
    matchup_result: &'a MatchupResult,
}

#[derive(Debug, Clone)]
enum CellMessage {
    Focus,
    Unfocus,
}

impl GridCell<'_> {
    pub fn new<'a>(matchup_result: &'a MatchupResult) -> GridCell<'a> {
        GridCell {
            matchup_result,
            is_selected: false,
        }
    }

    pub fn update(&mut self, message: CellMessage) {
        self.is_selected = match message {
            CellMessage::Focus => true,
            CellMessage::Unfocus => false,
        };
    }

    pub fn view(&self) -> Element<CellMessage> {
        button(Space::new(Length::Fill, Length::Fill))
            .on_press(CellMessage::Focus)
            .style(|_, _| {
                button::Style::default().with_background(Background::Color(Color::BLACK))
            })
            .padding(2)
            .width(20)
            .height(20)
            .into()
    }
}
