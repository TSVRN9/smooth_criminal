#![allow(dead_code)]

pub mod game;
pub mod strategies {
    pub mod classic;
    pub mod continuous;
    pub mod tsvrn9;
    pub mod utils;
}
pub mod widget {
    pub mod app;
    pub mod grid;
}

use widget::app::ResultsInspector;

use crate::game::*;

pub fn main() -> iced::Result {
    iced::application("Viewer", ResultsInspector::update, ResultsInspector::view)
        .run_with(ResultsInspector::new)

    // println!("Processing results...");

    // tokio::try_join!(
    //     write_raw_results_to_csv("results/raw.csv", &results),
    //     generate_performance_image("results/win_loss.png", &results, |GameResult(a, b)| a - b, 40, 20, 36.0),
    //     generate_performance_image("results/points.png", &results, |GameResult(a, _)| *a, 40, 20, 36.0),
    // )?;

    // println!("Done!");

    // Ok(())
}

