use std::{collections::HashMap, sync::Arc};

use iced::{
    widget::{container, row, text},
    window::{get_latest, maximize},
    Color, Element, Length, Task,
};
use rayon::prelude::*;

use crate::{
    colors::blend_colors,
    run_competition,
    strategies::{classic, continuous, tsvrn9},
    GameResult, MatchupResult,
};

use super::grid::{Grid, GridMessage};

#[derive(Default)]
pub enum ResultsInspector {
    #[default]
    Loading,
    Raw(RawState),
    Loaded(State),
}

pub struct State {
    grid: Grid,
    data: Data,
    selected_stat: &'static str,
    colors: Colors,
    filters: Vec<StatFilter>,
    cell_size: u16,

    selected_cell: Option<(usize, usize)>,
}

#[derive(Debug, Clone)]
pub struct RawState {
    data: Data,
    selected_stat: &'static str,
    filters: Vec<StatFilter>,
}

#[derive(Debug, Clone, Default)]
pub struct Data {
    strategy_names: Vec<&'static str>,
    matchup_results: Vec<MatchupResult>,
    stats: HashMap<&'static str, Arc<Stat>>,
}

#[derive(Debug, Clone)]
pub struct Colors {
    cell_colors: Vec<Color>,
    strategy_colors: Vec<Color>,
}

#[derive(Debug, Clone, Default)]
pub struct Stat {
    values: Vec<f64>,
    strategy_averages: Vec<f64>,
}

#[derive(Debug, Clone)]
pub enum StatFilter {
    HideRow(usize),
    HideColumn(usize),
}

#[derive(Debug, Clone)]
pub enum Message {
    Raw(Data),
    RecalculateColor,
    Loaded(Colors),
    GridMessage(GridMessage),
}

impl ResultsInspector {
    pub fn new() -> (ResultsInspector, Task<Message>) {
        (Self::Loading, Task::perform(load(), Message::Raw))
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Raw(data) => {
                if let ResultsInspector::Loading = self {
                    *self = ResultsInspector::Raw(RawState {
                        selected_stat: data.stats.keys().next().unwrap(),
                        data,
                        filters: vec![],
                    });

                    self.update(Message::RecalculateColor)
                } else {
                    panic!("Unexpected message");
                }
            }
            Message::RecalculateColor => {
                let stat = match self {
                    ResultsInspector::Loaded(state) => {
                        let selected_stat = state.selected_stat;
                        Arc::clone(&state.data.stats[selected_stat])
                    }
                    ResultsInspector::Raw(raw_state) => {
                        let selected_stat = raw_state.selected_stat;
                        Arc::clone(&raw_state.data.stats[selected_stat])
                    }
                    _ => panic!("Unexpected state"),
                };

                Task::perform(calculate_cell_and_strategy_colors(stat), Message::Loaded)
            }
            Message::Loaded(colors) => match self {
                ResultsInspector::Raw(raw_state) => {
                    let n = raw_state.data.strategy_names.len();

                    let mut new_state = State {
                        grid: Grid::new(n, n),
                        data: Default::default(),
                        selected_stat: raw_state.selected_stat,
                        colors,
                        filters: Default::default(),
                        selected_cell: None,
                        cell_size: 30,
                    };

                    std::mem::swap(&mut new_state.data, &mut raw_state.data);

                    *self = ResultsInspector::Loaded(new_state);

                    get_latest().then(|id| maximize(id.expect("No window found?"), true))
                }
                ResultsInspector::Loaded(state) => {
                    state.colors = colors;
                    Task::none()
                }
                _ => panic!("Unexpected State"),
            },
            _ => {
                if let ResultsInspector::Loaded(state) = self {
                    match message {
                        Message::GridMessage(grid_message) => {
                            match grid_message {
                                GridMessage::Focus(x, y) => {
                                    let previous_cell = state.selected_cell;
                                    state.selected_cell = Some((x, y));

                                    if let Some((x_previous, y_previous)) = previous_cell {
                                        state
                                            .grid
                                            .update(GridMessage::Unfocus(x_previous, y_previous));
                                    }
                                }
                                GridMessage::Unfocus(x, y) => {
                                    if state.selected_cell == Some((x, y)) {
                                        state.selected_cell = None;
                                    }
                                }
                            };

                            state.grid.update(grid_message);
                            Task::none()
                        }

                        Message::Raw(_) | Message::RecalculateColor | Message::Loaded(_) => {
                            panic!("Unreachable");
                        }
                    }
                } else {
                    panic!("Invalid State");
                }
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        match self {
            ResultsInspector::Loading | ResultsInspector::Raw(_) => container(
                text(if let ResultsInspector::Loading = self {
                    "Running simulations..."
                } else {
                    "Loading..."
                })
                .size(24),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center(Length::Fill)
            .into(),
            ResultsInspector::Loaded(state) => row!(state
                .grid
                .view(&state.colors.cell_colors, state.cell_size)
                .map(|m| Message::GridMessage(m)))
            .into(),
        }
    }
}

async fn load() -> Data {
    let strategies: Vec<_> = vec![classic::all(), continuous::all(), tsvrn9::all()]
        .into_iter()
        .flatten()
        .collect();

    let strategy_names = strategies.iter().map(|(name, _)| *name).collect();
    let grid_width = strategies.len();
    let matchup_results = run_competition(strategies).await;

    let (point_difference, points_per_round) = tokio::join!(
        calculate_stat(
            |MatchupResult {
                 overall_result: GameResult(a, b),
                 ..
             }| a - b,
            &matchup_results,
            grid_width
        ),
        calculate_stat(
            |MatchupResult {
                 overall_result: GameResult(a, _),
                 ..
             }| *a,
            &matchup_results,
            grid_width
        )
    );

    let mut stats = HashMap::new();

    stats.insert("Points per round", Arc::new(points_per_round));
    stats.insert("Point difference", Arc::new(point_difference));

    Data {
        strategy_names,
        matchup_results,
        stats,
    }
}

async fn calculate_stat(
    by: fn(&MatchupResult) -> f64,
    results: &Vec<MatchupResult>,
    grid_width: usize,
) -> Stat {
    let values: Vec<_> = results.iter().map(by).collect();

    let strategy_averages = values
        .chunks_exact(grid_width)
        .map(|d| d.iter().sum::<f64>() / d.len() as f64)
        .collect::<Vec<_>>();

    Stat {
        values,
        strategy_averages,
    }
}

async fn calculate_cell_and_strategy_colors(stat: Arc<Stat>) -> Colors {
    let average = stat.strategy_averages.iter().sum::<f64>() / stat.strategy_averages.len() as f64;

    let (cell_colors, strategy_colors) = tokio::join!(
        calculate_colors(average, &stat.values, Color::BLACK),
        calculate_colors(average, &stat.strategy_averages, Color::WHITE)
    );

    Colors {
        cell_colors,
        strategy_colors,
    }
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
    let to_blend_with = if deviation_percent > 0.0 {
        crate::colors::BLUE
    } else {
        crate::colors::RED
    };
    blend_colors(default, to_blend_with, deviation_percent.abs())
}
