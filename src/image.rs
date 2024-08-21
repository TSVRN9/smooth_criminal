use std::{error::Error, path::Path};

use ab_glyph::{FontRef, PxScale};
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
use tokio::fs;
use crate::{game::GameResult, NUM_ROUNDS};

pub async fn generate_performance_image(
    path: &'static str,
    results: &Vec<(&'static str, &'static str, GameResult)>,
    by: fn(&GameResult) -> f64,
    cell_size: u32,
    font_size: f32,
    padding: u32,
) -> Result<(), Box<dyn Error>> {
    let path = Path::new(path);
    let dir = path.parent().ok_or("Invalid Path")?;
    let create_dirs_task = tokio::spawn(fs::create_dir_all(dir));

    let scale = PxScale {
        x: font_size,
        y: font_size,
    };

    let font = {
        let data = include_bytes!("../assets/RobotoMono-Regular.ttf");
        FontRef::try_from_slice(data)
    }
    .expect("Could not load font!");

    let strategy_names = results
        .iter()
        .map(|(first_name, _, _)| String::from(*first_name))
        .dedup()
        .collect::<Vec<_>>();

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

    let (grid_colors, label_colors, labels) =
        calculate_grid_and_labels(|a| by(a), &strategy_names, results, grid_width).await;

    draw_color_grid_mut(
        &mut img,
        &grid_colors,
        grid_width,
        cell_size,
        (max_text_width + padding * 2) as i32,
        0,
    );

    draw_grid_labels_mut(
        &mut img,
        &label_colors,
        &labels,
        &font,
        &scale,
        max_text_width.try_into().unwrap(),
        cell_size.try_into().unwrap(),
        padding as i32,
        0,
    );

    // directory must be created before we save
    create_dirs_task.await??;
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
    labels: &Vec<String>,
    font: &FontRef,
    scale: &PxScale,
    width: i32,
    cell_size: i32,
    x: i32,
    y: i32,
) {
    assert_eq!(colors.len(), labels.len());
    for (i, (label, &color)) in labels.iter().zip(colors).enumerate() {
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

async fn calculate_grid_and_labels<F>(
    by: F,
    strategy_names: &Vec<String>,
    results: &Vec<(&str, &str, GameResult)>,
    grid_width: usize,
) -> (Vec<Rgb<u8>>, Vec<Rgb<u8>>, Vec<String>)
where
    F: Fn(&GameResult) -> f64,
{
    let values: Vec<_> = results
        .iter()
        .map(|(_, _, game_result)| by(game_result))
        .collect();

    let average_values = values
        .par_chunks_exact(grid_width)
        .map(|d| d.iter().sum::<f64>() / d.len() as f64)
        .collect::<Vec<_>>();

    let strategy_names = strategy_names.clone();

    let grid_colors_task = tokio::spawn(calculate_colors(&values, Rgb([0, 0, 0])));

    let labels_task = tokio::spawn(async move {
        let average_average_values = average_values.iter().sum::<f64>() / average_values.len() as f64;

        let deviance = average_values.par_iter().map(|v| v - average_average_values).collect::<Vec<_>>();

        let most_deviant_average = deviance
            .par_iter()
            .map(|d| d.abs())
            .max_by(|a, b| a.total_cmp(b))
            .expect("Could not get max deviant average, is it empty?");

        let label_colors = deviance
            .par_iter()
            .map(|d| {
                let deviancy = d / most_deviant_average;
                let default = Rgb([255, 255, 255]);
                calculate_color(deviancy, default)
            })
            .collect::<Vec<_>>();

        let labels: Vec<String> = strategy_names
            .iter()
            .zip(average_values)
            .map(|(name, avg_d)| {
                format!(
                    "{} {:+03}",
                    name,
                    (100.0 * avg_d / NUM_ROUNDS as f64).round()
                )
            })
            .collect();

        (label_colors, labels)
    });

    let (grid_colors, _) = grid_colors_task.await.unwrap();
    let (label_colors, labels) = labels_task.await.unwrap();

    (grid_colors, label_colors, labels)
}

async fn calculate_colors(values: &Vec<f64>, default: Rgb<u8>) -> (Vec<Rgb<u8>>, Vec<f64>) {
    let average_value = values.par_iter().sum::<f64>() / values.len() as f64;

    let deviance = values.par_iter().map(|v| v - average_value).collect::<Vec<_>>();

    let max_deviance = deviance
        .par_iter()
        .map(|f| f.abs())
        .max_by(|a, b| a.total_cmp(b))
        .expect("No maximum found, is the Vec empty?");

    let deviance_percents = deviance
        .par_iter()
        .map(|d| d / max_deviance)
        .collect::<Vec<_>>();

    let colors = deviance_percents
        .par_iter()
        .map(|d| {
            calculate_color(*d, default)
        })
        .collect::<Vec<_>>();

    (colors, deviance_percents)
}

fn calculate_color(deviation_percent: f64, default: Rgb<u8>) -> Rgb<u8> {
    let red = Rgb([255, 50, 50]);
    let blue = Rgb([50, 50, 255]);

    let to_blend_with = if deviation_percent > 0.0 { blue } else { red };

    default.map2(&to_blend_with, |d, b| {
        (d as f64 * (1.0 - deviation_percent.abs()) + b as f64 * deviation_percent.abs()) as u8
    })
}