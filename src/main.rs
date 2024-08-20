#![allow(dead_code)]

mod game;
mod strategies {
    pub mod classic;
    pub mod continuous;
    pub mod utils;
}

use crate::game::*;
use csv::Writer;
use imageproc::image::{Pixel, Rgb, RgbImage};
use std::{error::Error, fs, path::Path};
use strategies::{classic, continuous};

fn main() -> Result<(), Box<dyn Error>> {
    let strategies: Vec<(&'static str, Box<dyn Strategy>)> = vec![
        classic::all(),
        continuous::all()
    ]
    .into_iter()
    .flatten()
    .collect();

    println!("Running {} strategies...", strategies.len());
    let results = run_tournament(strategies);

    println!("Processing results...");
    write_results_to_csv("results/raw.csv", &results)?;
    generate_performance_image("results/performance.png", &results)?;
    println!("Done!");

    Ok(())
}

fn run_tournament(
    strategies: Vec<(&'static str, Box<dyn Strategy>)>,
) -> Vec<(&'static str, &'static str, GameResult)> {
    let r = strategies
        .iter()
        .map(|(first_name, first_strategy)| {
            strategies.iter().map(|(second_name, second_strategy)| {
                let mut first_strategy = dyn_clone::clone(&*first_strategy);
                let mut second_strategy = dyn_clone::clone(&*second_strategy);

                (
                    *first_name,
                    *second_name,
                    play_strategies(&mut first_strategy, &mut second_strategy),
                )
            })
        })
        .flatten()
        .collect();

    r
}

fn write_results_to_csv(
    path: &str,
    results: &Vec<(&'static str, &'static str, GameResult)>,
) -> Result<(), Box<dyn Error>> {
    let path = Path::new(path);
    let dir = path.parent().ok_or("Invalid Path")?;

    fs::create_dir_all(dir)?;

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

fn generate_performance_image(
    path: &str,
    results: &Vec<(&'static str, &'static str, GameResult)>,
) -> Result<(), Box<dyn Error>> {
    let path = Path::new(path);
    let dir = path.parent().ok_or("Invalid Path")?;
    fs::create_dir_all(dir)?;

    let strategies = results
        .iter()
        .map(|(first_name, _, _)| *first_name)
        .collect::<std::collections::HashSet<_>>(); // no itertools today!

    let differences = results.iter().map(|(_, _, GameResult(a, b))| a - b);

    let max_point_delta = differences
        .clone()
        .max_by(|a, b| a.total_cmp(b))
        .expect("No maximum found, is the Vec empty?");

    let cell_size = 20;
    let img_width = strategies.len() * cell_size;
    let img_height = strategies.len() * cell_size;

    let mut img = RgbImage::new(img_width as u32, img_height as u32);

    for (i, first_name) in strategies.iter().enumerate() {
        for (j, second_name) in strategies.iter().take(i + 1).enumerate() {
            if let Some((_, _, GameResult(first_score, second_score))) = results
                .iter()
                .find(|(f, s, _)| f == first_name && s == second_name)
            {
                let delta = first_score - second_score;
                let delta_percent = delta / max_point_delta / 2.0 + 0.5;

                let default = Rgb([251, 232, 255]); // nice shade of purple

                let red = Rgb([255, 20, 20]);
                let blue = Rgb([20, 20, 255]);

                let to_blend_with = if delta > 0.5 { blue } else { red };

                let color = default.map2(&to_blend_with, |d, b| {
                    (d as f64 * (1.0 - delta_percent) + b as f64 * delta_percent) as u8
                });

                for x in 0..cell_size {
                    for y in 0..cell_size {
                        img.put_pixel(
                            (j * cell_size + y) as u32,
                            (i * cell_size + x) as u32,
                            color,
                        );
                    }
                }
            }
        }
    }

    img.save(path)?;

    Ok(())
}
