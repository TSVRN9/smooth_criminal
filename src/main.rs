#![allow(dead_code)]

pub mod game;
pub mod strategies {
    pub mod classic;
    pub mod continuous;
    pub mod tsvrn9;
    pub mod utils;
}
pub mod widget {
    pub mod viewer;
    pub mod grid;
}
pub mod image;

use crate::game::*;
use crate::widget::viewer;

pub fn main() -> iced::Result {
    iced::run("Viewer", viewer::update, viewer::view)

    // let strategies: Vec<(&'static str, Box<dyn Strategy>)> =
    //     vec![classic::all(), continuous::all(), tsvrn9::all()]
    //         .into_iter()
    //         .flatten()
    //         .collect();

    // println!("Running {} strategies...", strategies.len());
    // let results = run_competition(strategies).await;

    // println!("Processing results...");

    // tokio::try_join!(
    //     write_raw_results_to_csv("results/raw.csv", &results),
    //     generate_performance_image("results/win_loss.png", &results, |GameResult(a, b)| a - b, 40, 20, 36.0),
    //     generate_performance_image("results/points.png", &results, |GameResult(a, _)| *a, 40, 20, 36.0),
    // )?;

    // println!("Done!");

    // Ok(())
}

async fn run_competition(
    strategies: Vec<(&'static str, Box<dyn Strategy>)>,
) -> Vec<(&'static str, &'static str, GameResult)> {
    let mut tasks = vec![];

    for (first_name, first_strategy) in strategies.iter() {
        for (second_name, second_strategy) in strategies.iter() {
            let first_name = *first_name;
            let second_name = *second_name;
            let mut first_strategy = dyn_clone::clone(&*first_strategy);
            let mut second_strategy = dyn_clone::clone(&*second_strategy);

            let task = tokio::spawn(async move {
                (
                    first_name,
                    second_name,
                    play_strategies(&mut first_strategy, &mut second_strategy),
                )
            });

            tasks.push(task);
        }
    }

    let mut results = vec![];
    for task in tasks {
        if let Ok(result) = task.await {
            results.push(result);
        }
    }

    results
}
