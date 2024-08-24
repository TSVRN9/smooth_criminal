use iced::{
    widget::{button, column, container, row, Space},
    Background, Color, Element, Length,
};

use crate::MatchupResult;

#[derive(Debug, Clone)]
pub enum GridMessage<'a> {
    Select(&'a MatchupResult),
}

#[derive(Default, Debug, Clone)]
pub enum GridStatus {
    #[default]
    Idle,
    Focused(MatchupResult),
}

#[derive(Debug, Clone)]
pub struct Grid {
    grid_cells: Vec<GridCell>,
    status: GridStatus,
}

impl Grid {
    pub fn new(matchup_results: Vec<MatchupResult>) -> Grid {
        let grid_cells = matchup_results.into_iter().map(GridCell::new).collect();
        Grid {
            grid_cells,
            status: GridStatus::Idle,
        }
    }

    pub fn update(&mut self, _message: GridMessage) {}

    pub fn view(&self) -> Element<GridMessage> {
        let n = (self.grid_cells.len() as f64).sqrt() as usize;

        let rows = self.grid_cells.chunks(n).map(|r| {
            row(r
                .iter()
                .map(|e| e.view().map(|_| GridMessage::Select(&e.matchup_result))))
            .into()
        });

        column(rows).into()
    }
}

#[derive(Debug, Clone)]
struct GridCell {
    is_selected: bool,
    matchup_result: MatchupResult,
}

#[derive(Debug, Clone)]
enum CellMessage {
    Focus,
    Unfocus,
}

impl GridCell {
    pub fn new(matchup_result: MatchupResult) -> GridCell {
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
        container(
            button(Space::new(Length::Fill, Length::Fill))
                .on_press(CellMessage::Focus)
                .style(|_, status| {
                    let bg_color = match status {
                        button::Status::Hovered => Color::from_rgb8(128, 128, 128),
                        _ => Color::BLACK
                    };

                    button::Style::default().with_background(Background::Color(bg_color))
                })
                .width(20)
                .height(20)
        ).padding(0).into()
    }
}
