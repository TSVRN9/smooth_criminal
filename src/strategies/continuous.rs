use crate::{from_functional, GameHistory, GameMove, Strategy, COOPERATE, DEFECT, P, R, S, T};

use super::utils;

pub fn all() -> Vec<(&'static str, Box<dyn Strategy>)> {
    vec![]
}

pub fn ambivalent(_: &GameHistory) -> f64 {
    0.5
}

pub fn ambivalent_suspicious(_: &GameHistory) -> f64 {
    0.75
}

pub fn ambivalent_relaxed(_: &GameHistory) -> f64 {
    0.25
}