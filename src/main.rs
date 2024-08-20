#![allow(dead_code)]

mod game;
mod strategies {
    pub mod classic;
    pub mod continuous;
    pub mod utils;
}

use crate::game::*;
use csv::Writer;
use imageproc::{
    drawing::{draw_text_mut, text_size},
    image::{Pixel, Rgb, RgbImage},
};
use itertools::Itertools;
use ab_glyph::{FontRef, PxScale};
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
        generate_performance_image("results/performance.png", &results)
    )?;

    println!("Done!");

    Ok(())
}

async fn run_competition(
    strategies: Vec<(&'static str, Box<dyn Strategy>)>,
) -> Vec<(&'static str, &'static str, GameResult)> {
    let mut tasks = vec![];

    for (i, (first_name, first_strategy)) in strategies.iter().enumerate() {
        for (second_name, second_strategy) in strategies.iter().take(i + 1) {
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
    }.expect("Could not load font!");

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

    let cell_size = 20;

    let scale = Scale::uniform((cell_size - 2) as f32);
    let padding = 10;

    // thanks chatgpt
    let max_text_width = strategy_names
        .iter()
        .map(|name| {
            calculate_text_width(name, &font, scale)
        })
        .max()
        .unwrap_or(0)
        + padding * 2; // Add some padding

    let img_width = strategy_names.len() * cell_size + max_text_width;
    let img_height = strategy_names.len() * cell_size;

    let mut img = RgbImage::new(img_width as u32, img_height as u32);

    for (i, first_name) in strategy_names.iter().enumerate() {
        for (j, second_name) in strategy_names.iter().take(i + 1).enumerate() {
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
                            (j * cell_size + y + max_text_width) as u32,
                            (i * cell_size + x) as u32,
                            color,
                        );
                    }
                }

                let text_width = calculate_text_width(first_name, &font, scale);

                draw_text_mut(
                    &mut img,
                    Rgb([255, 255, 255]),
                    (max_text_width - (padding + text_width)) as i32,
                    (i * cell_size) as i32,
                    scale,
                    &font,
                    text,
                )
            }
        }
    }

    // directory must be created before we save
    create_dirs_future.await?;
    img.save(path)?;

    Ok(())
}

fn calculate_text_width(text: &str, font: &Font, scale: Scale) -> usize {
    let v_metrics = font.v_metrics(scale);
    let glyphs: Vec<_> = font
        .layout(text, scale, rusttype::point(0.0, v_metrics.ascent))
        .collect();
    let width = glyphs
        .iter()
        .rev()
        .find_map(|g| g.pixel_bounding_box().map(|b| b.max.x as f32))
        .unwrap_or(0.0);
    width as usize
}