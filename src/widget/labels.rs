use std::collections::HashSet;

use iced::widget::*;
use iced::Alignment;
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
    const SPACING: u16 = 4;

    pub fn new() -> LabelList {
        Default::default()
    }

    pub fn update(&mut self, message: LabelListMessage) {
        match message {
            LabelListMessage::Focus(index) => self.selected_indicies.insert(index),
            LabelListMessage::Unfocus(index) => self.selected_indicies.remove(&index),
        };
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

        column(contents)
            .align_x(Alignment::End)
            .spacing(Self::SPACING)
            .into()
    }

    fn view_label(
        &self,
        index: usize,
        label: &'static str,
        color: Color,
        cell_size: u16,
    ) -> Element<LabelListMessage> {
        let is_selected = self.selected_indicies.contains(&index);
        let on_press_message = if is_selected {
            LabelListMessage::Unfocus(index)
        } else {
            LabelListMessage::Focus(index)
        };

        button(
            text(label)
                .style(move |_| text::Style { color: Some(color) })
                .size(cell_size - Self::SPACING * 2)
                .height(cell_size - Self::SPACING)
                .align_y(Alignment::Center),
        )
        .style(|_, _| button::Style {
            background: None,
            ..Default::default()
        })
        .padding(0)
        .on_press(on_press_message)
        .into()
    }
}
