use iced::{
    widget::{button, column, container, row, Space},
    Background, Color, Element, Length, Task,
};

use crate::MatchupResult;

#[derive(Debug, Clone)]
pub enum GridMessage {
    Select(usize),
    Loaded(stats::Statistics)

}

pub enum Grid {
    Loading,
    Loaded(GridState)
}

pub struct GridState {
    grid_cells: Vec<GridCell>,
    matchup_results: Vec<MatchupResult>,
    status: GridStatus,
}

#[derive(Default, Debug, Clone)]
pub enum GridStatus {
    #[default]
    Idle,
    Focused(MatchupResult),
}

impl Grid {
    pub fn new(matchup_results: Vec<MatchupResult>) -> (Grid, Task<GridMessage>) {
        (Grid::Loading, Task::perform(Grid::recalculate_statistics(matchup_results), GridMessage::Loaded))
    }

    async fn recalculate_statistics(matchup_results: Vec<MatchupResult>) -> Statistics {

    }

    pub fn update(&mut self, _message: GridMessage) {}

    pub fn view(&self) -> Element<GridMessage> {
        let n = (self.grid_cells.len() as f64).sqrt() as usize;

        let rows = self.grid_cells.chunks(n).map(|r| {
            row(r
                .iter()
                .enumerate()
                .map(|e| e.view().map(|(x, _)| GridMessage::Select())))
            .into()
        });

        column(rows).into()
    }
}

#[derive(Debug, Clone)]
struct GridCell {
    id: usize,
    is_selected: bool,
}

#[derive(Debug, Clone)]
enum CellMessage {
    Focus,
    Unfocus,
}

impl GridCell {
    pub fn new(id: usize) -> GridCell {
        GridCell {
            id,
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
                        _ => Color::BLACK,
                    };

                    button::Style::default().with_background(Background::Color(bg_color))
                })
                .width(20)
                .height(20),
        )
        .padding(0)
        .into()
    }
}

mod stats {
    #[derive(Debug, Clone)]
    pub struct Statistics {
        average: f64,
        average_values: Vec<f64>,
        colors: Vec<Color>
    }


    fn blend_colors(first: Color, second: Color, a: f32) -> Color {
        let x = 1.0 - a;
        Color::from_rgb(
            first.r * x + second.r * a,
            first.g * x + second.g + a,
            first.b * x + second.b * a,
        )
    }
}