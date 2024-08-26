use iced::{widget::text, Color, Element, Task};

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
    strategy_names: Vec<&'static str>,
    matchup_results: Vec<MatchupResult>,
    statistics: Statistics,
}

// can be recalculated
#[derive(Debug, Clone)]
pub struct Statistics {
    ppr: Vec<f64>,
    strategy_ppr: Vec<f64>,

    ppr_difference: Vec<f64>,
    strategy_ppr_difference: Vec<f64>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Loaded(Vec<&'static str>, Vec<MatchupResult>, Statistics),
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
                |(matchup_results, statistics)| Message::Loaded(matchup_results, statistics),
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
                let colors = state.statistics.ppr.
                state.grid.view().map(|_| Message::None)
            },
        }
    }
}

async fn load(
    strategies: Vec<(&'static str, Box<dyn Strategy>)>,
) -> (Vec<MatchupResult>, Vec<Statistics>) {
    let matchup_results = run_competition(strategies).await;
    let n = strategies.len();

    let statistics = calculate_grid_and_labels(by, strategy_names, results, n);

    (matchup_results, statistics)
}

async fn calculate_grid_and_labels<F>(
    by: F,
    strategy_names: &Vec<String>,
    results: &Vec<MatchupResult>,
    grid_width: usize,
) -> Statistics
where
    F: Fn(&MatchupResult) -> f64,
{
    let values: Vec<_> = results
        .iter()
        .map(|(_, _, game_result)| by(game_result))
        .collect();

    let strategy_averages = values
        .par_chunks_exact(grid_width)
        .map(|d| d.iter().sum::<f64>() / d.len() as f64)
        .collect::<Vec<_>>();

    let average = values.iter().sum::<f64>() / values.len() as f64;

    let strategy_names = strategy_names.clone();

    let (grid_colors, label_colors) = tokio::join!(
        calculate_colors(average, &values, BLACK),
        calculate_colors(average, &strategy_averages, WHITE)
    );

    let labels: Vec<String> = strategy_names
        .iter()
        .zip(strategy_averages)
        .map(|(name, avg_d)| {
            format!(
                "{} {:+03}",
                name,
                (100.0 * avg_d / NUM_ROUNDS as f64).round()
            )
        })
        .collect();

    (grid_colors, label_colors, labels)
}