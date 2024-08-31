use iced::widget::column;
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
        // could be left blank tbh
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

        let title = column!(
            text!("{} - {:.2}", first_name, overall_result.0,).size(36),
            text!("vs").size(18),
            text!("{} - {:.2}", second_name, overall_result.1,).size(36)
        )
        .align_x(Alignment::Center);

        let visualization = column!(
            self.grid
                .view(&colors, cell_size)
                .map(MatchInspectorMessage::GridMessage)
        ,)
        .width(Length::Fill)
        .align_x(Alignment::Center);

        let content = column!(title, scrollable(visualization).width(Length::Fill))
            .align_x(Alignment::Center)
            .width(Length::Fill);

        content.padding(4).into()
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
