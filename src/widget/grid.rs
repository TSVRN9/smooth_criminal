use iced::{
    widget::{button, column, container, keyed_column, row, Space},
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

    pub fn view(&self) -> Element<GridMessage> {
        let rows = self
            .cells
            .chunks(self.width)
            .enumerate()
            .map(|(row_index, row_cells)| self.view_row(row_index, row_cells));

        column(rows).into()
    }

    fn view_row(&'a self, row_index: usize, row_cells: &'a [Cell]) -> Element<GridMessage> {
        let cells = row_cells
            .iter()
            .enumerate()
            .map(|(col_index, cell)| self.view_cell(row_index, col_index, cell));

        row(cells).into()
    }

    fn view_cell(&'a self, row: usize, col: usize, cell: &'a Cell) -> Element<GridMessage> {
        cell.view().map(move |m| match m {
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

    pub fn view(&self) -> Element<CellMessage> {
        container(
            button(Space::new(Length::Fill, Length::Fill))
                .on_press(CellMessage::Focused)
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
