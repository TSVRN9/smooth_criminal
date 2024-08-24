use iced::{widget::text, Element, Task};

use crate::{
    run_competition,
    strategies::{classic, continuous, tsvrn9},
    MatchupResult, Strategy,
};

use super::grid::Grid;

#[derive(Default)]
pub enum ResultsInspector<'a> {
    #[default]
    Loading,
    Loaded(State<'a>),
}

pub struct State<'a> {
    grid: Grid<'a>,
    matchup_results: Vec<MatchupResult>,
}

#[derive(Debug, Clone)]
pub enum Message {
    None,
    Loaded(Vec<MatchupResult>),
}

impl<'a> ResultsInspector<'a> {
    pub fn new() -> (ResultsInspector<'a>, Task<Message>) {
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

    pub fn update(&mut self, _: Message) {}
}
