use iced::{
    widget::{button, column, container, row, Space},
    Background, Border, Color, Element, Length,
};

#[derive(Debug, Clone)]
pub enum GridMessage {
    Focus(usize, usize),
    Unfocus(usize, usize),
}

pub struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl<'a> Grid {
    pub fn new(width: usize, height: usize) -> Grid {
        Grid {
            width,
            height,
            cells: (0..(width * height))
                .into_iter()
                .map(|i| Cell::new(i))
                .collect(),
        }
    }

    pub fn update(&mut self, message: GridMessage) {
        match message {
            GridMessage::Focus(x, y) => {
                let cell = self.get_cell_mut(x, y);
                cell.update(CellMessage::Focus)
            }
            GridMessage::Unfocus(x, y) => {
                let cell = self.get_cell_mut(x, y);
                cell.update(CellMessage::Unfocus)
            }
        }
    }

    fn get_cell_mut(&mut self, x: usize, y: usize) -> &mut Cell {
        let i = self.flatten_indicies(x, y);
        self.cells.get_mut(i).unwrap()
    }

    fn flatten_indicies(&self, x: usize, y: usize) -> usize {
        x * self.width + y
    }

    pub fn view(&self, colors: &Vec<Color>) -> Element<GridMessage> {
        let rows = self
            .cells
            .chunks(self.width)
            .zip(colors.chunks(self.width))
            .enumerate()
            .map(|(row_index, (row_cells, row_colors))| {
                self.view_row(row_index, row_cells, row_colors)
            });

        column(rows).into()
    }

    fn view_row(
        &'a self,
        row_index: usize,
        row_cells: &'a [Cell],
        row_colors: &[Color],
    ) -> Element<GridMessage> {
        let cells = row_cells
            .iter()
            .zip(row_colors)
            .enumerate()
            .map(|(col_index, (cell, &color))| self.view_cell(row_index, col_index, cell, color));

        row(cells).into()
    }

    fn view_cell(
        &'a self,
        row: usize,
        col: usize,
        cell: &'a Cell,
        color: Color,
    ) -> Element<GridMessage> {
        cell.view(color, row == col).map(move |m| match m {
            CellMessage::Focus => GridMessage::Focus(row, col),
            CellMessage::Unfocus => GridMessage::Unfocus(row, col),
            CellMessage::ToggleFocus => {
                if cell.is_selected {
                    GridMessage::Unfocus(row, col)
                } else {
                    GridMessage::Focus(row, col)
                }
            }
        })
    }
}

#[derive(Debug, Clone)]
struct Cell {
    id: usize,
    is_selected: bool,
}

#[derive(Debug, Clone)]
enum CellMessage {
    ToggleFocus,
    Focus,
    Unfocus,
}

impl Cell {
    pub fn new(id: usize) -> Cell {
        Cell {
            id,
            is_selected: false,
        }
    }

    pub fn update(&mut self, message: CellMessage) {
        self.is_selected = match message {
            CellMessage::ToggleFocus => !self.is_selected,
            CellMessage::Focus => true,
            CellMessage::Unfocus => false,
        };
    }

    pub fn view(&self, color: Color, show_border: bool) -> Element<CellMessage> {
        container(
            button(Space::new(Length::Fill, Length::Fill))
                .on_press(CellMessage::ToggleFocus)
                .style(move |_, status| {
                    use crate::colors::blend_colors;
                    use button::{Status, Style};

                    let tint = match status {
                        Status::Hovered | Status::Pressed => 0.2,
                        _ => 0.0,
                    };

                    let bg_color = blend_colors(color, Color::WHITE, tint);

                    Style {
                        background: Some(Background::Color(bg_color)),
                        border: if self.is_selected {
                            Border {
                                color: blend_colors(bg_color, crate::colors::YELLOW, 0.6),
                                width: 2.0,
                                radius: Default::default(),
                            }
                        } else if show_border {
                            Border {
                                color: blend_colors(bg_color, Color::WHITE, 0.2),
                                width: 2.0,
                                radius: Default::default(),
                            }
                        } else {
                            Default::default()
                        },
                        ..button::Style::default()
                    }
                })
                .width(30)
                .height(30),
        )
        .padding(0)
        .into()
    }
}
