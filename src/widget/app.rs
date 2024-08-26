use std::collections::HashMap;

use iced::{widget::text, Color, Element, Task};
use rayon::prelude::*;

use crate::{
    colors::blend_colors, run_competition, strategies::{classic, continuous, tsvrn9}, MatchupResult, Strategy
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
    strategy_names: Vec<&'static str>,
    matchup_results: Vec<MatchupResult>,
    stats: HashMap<&'static str, Stat>,
    selected_stat: &'static str,
    filters: Vec<StatFilter>,
}

#[derive(Debug, Clone)]
pub struct Stat {
    values: Vec<f64>,
    strategy_averages: Vec<f64>
}

pub enum StatFilter {
    HideRow(usize),
    HideColumn(usize),
}


#[derive(Debug, Clone)]
pub enum Message {
    Loaded(Vec<&'static str>, Vec<MatchupResult>, HashMap<&'static str, Stat>),
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
            Task::perform(
                load(strategies),
                |(strategy_names, matchup_results, statistics)| Message::Loaded(matchup_results, statistics),
            ),
        )
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Loaded(strategy_names, matchup_results, statistics) => {
                let n = strategy_names.len();
                *self = ResultsInspector::Loaded(State {
                    grid: Grid::new(n, n),
                    strategy_names, matchup_results, statistics
                })
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        match self {
            ResultsInspector::Loading => text("Loading...").into(),
            ResultsInspector::Loaded(state) => {
                let (cell_colors, _label_colors) = calculate_cell_and_strategy_colors(by, results, grid_width)
                state.grid.view().map(|_| Message::None)
            },
        }
    }
}

async fn load(
    strategies: Vec<(&'static str, Box<dyn Strategy>)>,
) -> (Vec<&'static str>, Vec<MatchupResult>, Vec<Stat>) {
    let matchup_results = run_competition(strategies).await;
    let n = strategies.len();

    let statistics = calculate_stat()

    (matchup_results, statistics)
}

async fn calculate_stat<F>(
    by: F,
    results: &Vec<MatchupResult>,
    grid_width: usize,
) -> Stat
where
    F: Fn(&MatchupResult) -> f64,
{
    let values: Vec<_> = results
        .iter()
        .map(by)
        .collect();

    let strategy_averages = values
        .chunks_exact(grid_width)
        .map(|d| d.iter().sum::<f64>() / d.len() as f64)
        .collect::<Vec<_>>();

    Stat {
        values, strategy_averages
    }
}

async fn calculate_cell_and_strategy_colors(
    stat: &Stat,
) -> (Vec<Color>, Vec<Color>)
{
    let average = stat.strategy_averages.iter().sum::<f64>() / stat.values.len() as f64;

    let (cell_colors, label_colors) = tokio::join!(
        calculate_colors(average, &stat.values, Color::BLACK),
        calculate_colors(average, &stat.strategy_averages, Color::WHITE)
    );

    (cell_colors, label_colors)
}

async fn calculate_colors(standard: f64, values: &Vec<f64>, default: Color) -> Vec<Color> {
    let deviance = values.par_iter().map(|v| v - standard).collect::<Vec<_>>();

    let max_deviance = deviance
        .par_iter()
        .map(|f| f.abs())
        .max_by(|a, b| a.total_cmp(b))
        .expect("No maximum found, is the Vec empty?");

    let deviance_percents = deviance
        .par_iter()
        .map(|d| d / max_deviance)
        .collect::<Vec<_>>();

    let colors = deviance_percents
        .par_iter()
        .map(|d| calculate_color(*d as f32, default))
        .collect::<Vec<_>>();

    colors
}

fn calculate_color(deviation_percent: f32, default: Color) -> Color {
    let to_blend_with = if deviation_percent > 0.0 { crate::colors::BLUE } else { crate::colors::RED };
    blend_colors(default, to_blend_with, deviation_percent.abs())
}