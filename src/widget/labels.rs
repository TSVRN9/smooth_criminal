use std::collections::HashSet;

use iced::widget::*;
use iced::Color;
use iced::Element;

#[derive(Debug, Clone)]
pub enum LabelListMessage {
    Focus(usize),
    Unfocus(usize),
}

#[derive(Debug, Default)]
pub struct LabelList {
    selected_indicies: HashSet<usize>, // prob fast enough
}

impl LabelList {
    pub fn new() -> LabelList {
        Default::default()
    }

    pub fn update(&mut self, _message: LabelListMessage) {
        todo!()
    }

    pub fn view(
        &self,
        labels: &Vec<&'static str>,
        label_colors: &Vec<Color>,
        cell_size: u16,
    ) -> Element<LabelListMessage> {
        let contents = labels
            .iter()
            .zip(label_colors)
            .enumerate()
            .map(|(index, (&label, &color))| self.view_label(index, label, color, cell_size));

        row(contents).into()
    }

    fn view_label(
        &self,
        index: usize,
        label: &'static str,
        color: Color,
        cell_size: u16,
    ) -> Element<LabelListMessage> {
        use text::Style;

        let is_selected = self.selected_indicies.contains(&index);
        let on_press_mesage = if is_selected {
            LabelListMessage::Unfocus(index)
        } else {
            LabelListMessage::Focus(index)
        };

        button(
            text(label)
                .style(move |_| Style { color: Some(color) })
                .size(cell_size),
        )
        .on_press(on_press_mesage)
        .into()
    }
}
