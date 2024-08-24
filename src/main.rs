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

use widget::grid::Grid;

use crate::game::*;
use crate::widget::viewer;

pub fn main() -> iced::Result {
    let x = 3;

    let y = &x;

    iced::application("Viewer", Grid::update, Grid::view)
        

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

