#![allow(dead_code)]

mod game;
mod strategies {
    pub mod classic;
    pub mod continuous;
    pub mod utils;
}

use strategies::{classic, continuous};
use crate::game::*;
use csv::Writer;
use plotters::prelude::*;
use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let strategies: Vec<(&'static str, Box<dyn Strategy>)> =
        vec![classic::all(), continuous::all()]
            .into_iter()
            .flatten()
            .collect();

    let results = run_tournament(strategies);

    write_results_to_csv(&results)?;
    generate_performance_image(&results)?;

    Ok(())
}


fn run_tournament(strategies: Vec<(&'static str, Box<dyn Strategy>)>) -> Vec<(&'static str, &'static str, GameResult)>{
    let r = strategies.iter().map(|(first_name, first_strategy)| {
        strategies.iter().map(|(second_name, second_strategy)| {
            let mut first_strategy = dyn_clone::clone(&*first_strategy);
            let mut second_strategy = dyn_clone::clone(&*second_strategy);

            (*first_name, *second_name, play_strategies(&mut first_strategy, &mut second_strategy))
        })
    }).flatten().collect();

    r
}

fn write_results_to_csv(results: &Vec<(&'static str, &'static str, GameResult)>) -> Result<(), Box<dyn Error>> {
    let dir = "results";
    let filename = "results.csv";

    fs::create_dir_all("results")?;

    let mut wtr = Writer::from_path(format!("{}/{}", dir, filename))?;
    wtr.write_record(&["First Strategy", "Second Strategy", "First Score", "Second Score"])?;

    for (first_name, second_name, GameResult(first_score, second_score)) in results {
        wtr.write_record(&[first_name, second_name, &first_score.to_string().as_str(), &second_score.to_string().as_str()])?;
    }

    wtr.flush()?;
    Ok(())
}

fn generate_performance_image(results: &Vec<(&'static str, &'static str, GameResult)>) -> Result<(), Box<dyn Error>> {
    todo!();
}