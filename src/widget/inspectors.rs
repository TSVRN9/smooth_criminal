use iced::widget::column;
use iced::widget::container;
use iced::widget::scrollable;
use iced::widget::text;
use iced::Alignment;
use iced::Color;
use iced::Element;
use iced::Length;

use crate::{MatchupResult, NUM_ROUNDS};

use super::grid::Grid;
use super::grid::GridMessage;

#[derive(Debug, Clone)]
pub enum MatchInspectorMessage {
    GridMessage(GridMessage),
}

pub struct MatchInspector {
    grid: Grid,
}

impl Default for MatchInspector {
    fn default() -> Self {
        Self {
            grid: Grid::new(2, NUM_ROUNDS, false),
        }
    }
}

impl MatchInspector {
    pub fn update(&mut self, _message: MatchInspectorMessage) {
        todo!()
    }

    pub fn view(
        &self,
        matchup_result: &MatchupResult,
        _round_index: Option<usize>,
        cell_size: u16,
    ) -> Element<MatchInspectorMessage> {
        let MatchupResult {
            first_name,
            second_name,
            overall_result,
            history,
        } = matchup_result;

        let colors = history
            .iter()
            .flat_map(|m| [m.0, m.1])
            .map(Self::calculate_move_color)
            .collect();

        let title = text!(
            "{} - {:.2}\nvs\n{:.2} - {}",
            first_name,
            overall_result.0,
            overall_result.1,
            second_name
        )
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .size(25);
        let visualization = container(
            self.grid
                .view(&colors, cell_size)
                .map(MatchInspectorMessage::GridMessage),
        )
        .center(Length::Fill)
        .width(Length::Fill);

        let content = column!(title, scrollable(visualization).width(Length::Fill))
            .align_x(Alignment::Center)
            .width(Length::Fill);

        content.into()
    }

    fn calculate_move_color(mv: f64) -> Color {
        let a = (mv as f32 - 0.5) * 2.0;

        let to_blend_with = if a > 0.0 {
            crate::colors::RED // defection
        } else {
            crate::colors::BLUE // cooperate
        };
        crate::colors::blend_colors(crate::colors::YELLOW, to_blend_with, a.abs())
    }
}
