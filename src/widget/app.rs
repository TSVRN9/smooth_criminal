use iced::{widget::text, Element, Task};

use crate::{
    run_competition,
    strategies::{classic, continuous, tsvrn9},
    MatchupResult, Strategy,
};

use super::grid::Grid;

#[derive(Default)]
pub enum ResultsInspector {
    #[default]
    Loading,
    Loaded(State),
}

pub struct State {
    grid: Grid,
}

#[derive(Debug, Clone)]
pub enum Message {
    None,
    Loaded(Vec<MatchupResult>),
}

impl ResultsInspector {
    pub fn new() -> (ResultsInspector, Task<Message>) {
        let strategies: Vec<(&'static str, Box<dyn Strategy>)> =
            vec![classic::all(), continuous::all(), tsvrn9::all()]
                .into_iter()
                .flatten()
                .collect();

        (
            Self::Loading,
            Task::perform(run_competition(strategies), Message::Loaded),
        )
    }

    pub fn view(&self) -> Element<Message> {
        match self {
            ResultsInspector::Loading => text("Loading...").into(),
            ResultsInspector::Loaded(state) => state.grid.view().map(|_| Message::None),
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::None => {},
            Message::Loaded(matchup_results) => {
                *self = ResultsInspector::Loaded(State {
                    grid: Grid::new(matchup_results)
                })
            }
        }
    }
}
