use iced::{
    widget::{button, column, container, row, Space},
    Background, Color, Element, Length,
};

#[derive(Debug, Clone)]
pub enum GridMessage {
    Focused(usize, usize),
    Unfocused(usize, usize),
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
            GridMessage::Focused(x, y) => {
                let cell = self.get_cell_mut(x, y);
                cell.update(CellMessage::Focused)
            }
            GridMessage::Unfocused(x, y) => {
                let cell = self.get_cell_mut(x, y);
                cell.update(CellMessage::Unfocused)
            }
        }
    }

    fn get_cell_mut(&mut self, x: usize, y: usize) -> &mut Cell {
        let i = self.flatten_indicies(x, y);
        self.cells.get_mut(i).unwrap()
    }

    fn flatten_indicies(&self, x: usize, y: usize) -> usize {
        y * self.width + x
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
        cell.view(color).map(move |m| match m {
            CellMessage::Focused => GridMessage::Focused(row, col),
            CellMessage::Unfocused => GridMessage::Unfocused(row, col),
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
    Focused,
    Unfocused,
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
            CellMessage::Focused => true,
            CellMessage::Unfocused => false,
        };
    }

    pub fn view(&self, color: Color) -> Element<CellMessage> {
        container(
            button(Space::new(Length::Fill, Length::Fill))
                .on_press(CellMessage::Focused)
                .style(move |_, status| {
                    use button::Status;
                    let tint = match status {
                        Status::Hovered | Status::Pressed => 0.2,
                        _ => 0.0,
                    };

                    let bg_color = blend_colors(color, Color::WHITE, tint);
                    button::Style::default().with_background(Background::Color(bg_color))
                })
                .width(20)
                .height(20),
        )
        .padding(0)
        .into()
    }
}

fn blend_colors(first: Color, second: Color, a: f32) -> Color {
    let x = 1.0 - a;
    Color::from_rgb(
        first.r * x + second.r * a,
        first.g * x + second.g + a,
        first.b * x + second.b * a,
    )
}
