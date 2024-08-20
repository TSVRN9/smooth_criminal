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
use rayon::{
    iter::{IntoParallelRefIterator, ParallelIterator},
    slice::ParallelSlice,
};
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
        generate_performance_image("results/performance.png", &results, 40, 30.0, 20)
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
    cell_size: u32,
    font_size: f32,
    padding: u32,
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

    let scale = PxScale {
        x: font_size,
        y: font_size,
    };

    // thanks for nothing chatgpt
    let max_text_width = strategy_names
        .iter()
        .map(|name| text_size(scale, &font, name).0)
        .max()
        .unwrap_or(0)
        + text_size(scale, &font, " 000").0
        + padding * 2; // Add some padding

    let img_width = strategy_names.len() as u32 * cell_size + max_text_width;
    let img_height = strategy_names.len() as u32 * cell_size;

    let mut img = RgbImage::new(img_width, img_height);

    let grid_width = strategy_names.len();

    let deltas: Vec<_> = results
        .iter()
        .map(|(_, _, GameResult(a, b))| a - b)
        .collect();

    let max_point_delta = deltas
        .par_iter()
        .map(|f| f.abs())
        .max_by(|a, b| a.total_cmp(b))
        .expect("No maximum found, is the Vec empty?");

    let delta_percents = deltas
        .par_iter()
        .map(|d| d / max_point_delta)
        .collect::<Vec<_>>();

    let grid_colors = delta_percents
        .par_iter()
        .map(|d| {
            let default = Rgb([0, 0, 0]);
            calculate_color(*d, default)
        })
        .collect();

    let average_deltas = deltas
        .par_chunks_exact(grid_width)
        .map(|d| d.iter().sum::<f64>() / d.len() as f64)
        .collect::<Vec<_>>();

    let most_deviant_average = average_deltas
        .par_iter()
        .map(|f| f.abs())
        .max_by(|a, b| a.total_cmp(b))
        .expect("Could not get max deviant average, is it empty?");

    let label_colors = average_deltas.par_iter()
        .map(|d| {
            let deviancy = d / most_deviant_average;
            let default = Rgb([255, 255, 255]);
            calculate_color(deviancy, default)
        })
        .collect();

    draw_color_grid_mut(
        &mut img,
        &grid_colors,
        grid_width,
        cell_size,
        max_text_width as i32,
        0,
    );

    draw_grid_labels_mut(
        &mut img,
        &label_colors,
        &strategy_names,
        &font,
        &scale,
        max_text_width.try_into().unwrap(),
        (cell_size + padding).try_into().unwrap(),
        padding.try_into().unwrap(),
        0,
    );

    // directory must be created before we save
    create_dirs_future.await?;
    img.save(path)?;

    Ok(())
}

fn draw_color_grid_mut(
    image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    colors: &Vec<Rgb<u8>>,
    colors_width: usize,
    cell_size: u32,
    x: i32,
    y: i32,
) {
    for (i, color) in colors.iter().enumerate() {
        let j = i % colors_width;
        let i = i / colors_width;

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

fn draw_grid_labels_mut(
    image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    colors: &Vec<Rgb<u8>>,
    labels: &Vec<&str>,
    font: &FontRef,
    scale: &PxScale,
    width: i32,
    cell_size: i32,
    x: i32,
    y: i32,
) {
    assert_eq!(colors.len(), labels.len());
    for (i, (&label, &color)) in labels.iter().zip(colors).enumerate() {
        let text_width = text_size(*scale, &font, label).0 as i32;

        draw_text_mut(
            image,
            color,
            x + width - text_width,
            y + (i * cell_size as usize) as i32,
            *scale,
            &font,
            label,
        )
    }
}

fn calculate_color(deviation_percent: f64, default: Rgb<u8>) -> Rgb<u8> {
    let red = Rgb([255, 50, 50]);
    let blue = Rgb([50, 50, 255]);

    let to_blend_with = if deviation_percent > 0.0 { blue } else { red };

    default.map2(&to_blend_with, |d, b| {
        (d as f64 * (1.0 - deviation_percent.abs()) + b as f64 * deviation_percent.abs()) as u8
    })
}
