#![allow(dead_code)]

mod game;
mod strategies {
    pub mod classic;
    pub mod continuous;
    pub mod utils;
}

use crate::game::*;
use ab_glyph::{FontRef, PxScale};
use csv::Writer;
use imageproc::{
    drawing::{self, draw_text_mut, text_size},
    image::{ImageBuffer, Pixel, Rgb, RgbImage},
    rect::Rect,
};
use itertools::Itertools;
use rayon::{iter::{IntoParallelRefIterator, ParallelIterator}, slice::ParallelSlice};
use std::{cmp::Ordering, error::Error, path::Path};
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
        generate_performance_image("results/performance.png", &results)
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

async fn generate_performance_image(
    path: &str,
    results: &Vec<(&'static str, &'static str, GameResult)>,
) -> Result<(), Box<dyn Error>> {
    let path = Path::new(path);
    let dir = path.parent().ok_or("Invalid Path")?;
    let create_dirs_future = fs::create_dir_all(dir);

    let font = {
        let data = include_bytes!("../assets/RobotoMono-Regular.ttf");
        FontRef::try_from_slice(data)
    }
    .expect("Could not load font!");

    let strategy_names = results
        .iter()
        .map(|(first_name, _, _)| *first_name)
        .dedup()
        .collect::<Vec<_>>();

    let differences = results.iter().map(|(_, _, GameResult(a, b))| a - b);

    let max_point_delta = differences
        .clone()
        .max_by(|a, b| a.total_cmp(b))
        .expect("No maximum found, is the Vec empty?");

    let cell_size = 40;

    let font_size = 36.0;
    let scale = PxScale {
        x: font_size,
        y: font_size,
    };
    let padding = 20;

    // thanks chatgpt
    let max_text_width = strategy_names
        .iter()
        .map(|name| text_size(scale, &font, name).0)
        .max()
        .unwrap_or(0)
        + text_size(scale, &font, " 000").0
        + padding * 2; // Add some padding

    let img_width = strategy_names.len() as u32 * cell_size + max_text_width;
    let img_height = strategy_names.len() as u32 * cell_size;

    let grid_width = strategy_names.len();

    let mut img = RgbImage::new(img_width, img_height);

    let colors = results.par_iter().map(
        |(_, _, GameResult(first_score, second_score))| {
            let delta = first_score - second_score;
            let delta_percent = delta / max_point_delta;
            let default = Rgb([0, 0, 0]);

            calculate_color(delta_percent, default)
        }
    ).collect();

    // TODO let records = results.as_slice().par_chunks(grid_width).map 

    // for _ in strategy_names.iter().enumerate() {
    //     let mut points = 0.0;
    //     let mut wins = 0;
    //     let mut loss = 0;
    //     let mut tie = 0;

    //     for (j, second_name) in strategy_names.iter().enumerate() {
    //         if let Some((_, _, GameResult(first_score, second_score))) = results
    //             .iter()
    //             .find(|(f, s, _)| f == first_name && s == second_name)
    //         {
    //             let color = {
    //                 let delta = first_score - second_score;
    //                 let delta_percent = delta / max_point_delta;
    //                 let default = Rgb([0, 0, 0]);

    //                 calculate_color(delta_percent, default)
    //             };

    //             colors.push(color);

    //             points += first_score;

    //             match first_score.total_cmp(second_score) {
    //                 Ordering::Greater => wins += 1,
    //                 Ordering::Less => loss += 1,
    //                 Ordering::Equal => tie += 1,
    //             };
    //         }
    //     }

    //     let ppr = points / (NUM_ROUNDS * strategy_names.len()) as f64;
    //     pprs.push(ppr);
    //     record.push((wins, loss, tie));
    // }

    draw_color_grid_mut(
        &mut img,
        &colors,
        grid_width,
        cell_size,
        max_text_width as i32,
        0,
    );

    // let average_ppr = pprs.iter().sum::<f64>() / pprs.len() as f64;
    // let max_outlier_ppr = pprs
    //     .iter()
    //     .map(|m| (m - average_ppr).abs())
    //     .max_by(|a, b| a.total_cmp(b))
    //     .expect("No max outlier ppr found, is strategies array empty?");

    // for (i, first_name) in strategy_names.iter().enumerate() {
    //     let ppr = pprs.get(i).unwrap();

    //     let text = format!("{} {}", *first_name, (ppr * 100.0).round() as usize);
    //     let text_width = text_size(scale, &font, &text).0;
    //     let color = {
    //         let delta = (ppr - average_ppr).clamp(-max_outlier_ppr * 0.95, max_outlier_ppr * 0.95);
    //         let delta_percent = delta / max_outlier_ppr;
    //         calculate_color(delta_percent, Rgb([255, 255, 255]))
    //     };

    //     draw_text_mut(
    //         &mut img,
    //         color,
    //         (max_text_width - (padding + text_width)) as i32,
    //         (i * cell_size as usize) as i32,
    //         scale,
    //         &font,
    //         &text,
    //     )
    // }

    // directory must be created before we save
    create_dirs_future.await?;
    img.save(path)?;

    Ok(())
}

fn draw_color_grid_mut(
    image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    colors: &Vec<Rgb<u8>>,
    width: usize,
    cell_size: u32,
    x: i32,
    y: i32,
) {
    for (i, color) in colors.iter().enumerate() {
        let j = i % width;
        let i = i / width;

        drawing::draw_filled_rect_mut(
            image,
            Rect::at(
                x + (j as u32 * cell_size) as i32,
                y + (i as u32 * cell_size) as i32,
            )
            .of_size(cell_size, cell_size),
            *color,
        );
    }
}

fn calculate_color(delta_percent: f64, default: Rgb<u8>) -> Rgb<u8> {
    let red = Rgb([255, 50, 50]);
    let blue = Rgb([50, 50, 255]);

    let to_blend_with = if delta_percent > 0.0 { blue } else { red };

    default.map2(&to_blend_with, |d, b| {
        (d as f64 * (1.0 - delta_percent.abs()) + b as f64 * delta_percent.abs()) as u8
    })
}
