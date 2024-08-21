#![allow(dead_code)]

pub mod game;
pub mod strategies {
    pub mod classic;
    pub mod continuous;
    pub mod utils;
}
pub mod image;

use crate::game::*;
use crate::image::generate_performance_image;
use csv::Writer;
use std::{error::Error, path::Path};
use strategies::{classic, continuous};
use tokio::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let strategies: Vec<(&'static str, Box<dyn Strategy>)> =
        vec![classic::all(), continuous::all()]
            .into_iter()
            .flatten()
            .collect();

    println!("Running {} strategies...", strategies.len());
    let results = run_competition(strategies).await;

    println!("Processing results...");

    tokio::try_join!(
        write_raw_results_to_csv("results/raw.csv", &results),
        generate_performance_image("results/win_loss.png", &results, |GameResult(a, b)| a - b, 40, 20, 36.0),
        generate_performance_image("results/points.png", &results, |GameResult(a, _)| *a, 40, 20, 36.0),
    )?;

    println!("Done!");

    Ok(())
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

async fn write_raw_results_to_csv(
    path: &str,
    results: &Vec<(&'static str, &'static str, GameResult)>,
) -> Result<(), Box<dyn Error>> {
    let path = Path::new(path);
    let dir = path.parent().ok_or("Invalid Path")?;

    fs::create_dir_all(dir).await?;

    let mut wtr = Writer::from_path(path)?;
    wtr.write_record(&[
        "First Strategy",
        "Second Strategy",
        "First Score",
        "Second Score",
    ])?;

    for (first_name, second_name, GameResult(first_score, second_score)) in results {
        wtr.write_record(&[
            first_name,
            second_name,
            &first_score.to_string().as_str(),
            &second_score.to_string().as_str(),
        ])?;
    }

    wtr.flush()?;
    Ok(())
}