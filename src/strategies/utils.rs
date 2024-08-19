const DEFECTION_THRESHOLD: f64 = 0.5; // 0.0 is COOPERATE

use crate::{GameMove, COOPERATE, DEFECT};

pub fn to_opponent_move(GameMove(_, opponent_move): &GameMove) -> f64 {
    *opponent_move
}

pub fn to_my_move(GameMove(my_move, _): &GameMove) -> f64 {
    *my_move
}

pub fn is_cooperation(m: &f64) -> bool {
    m < &DEFECTION_THRESHOLD
}

pub fn is_defection(m: &f64) -> bool {
    m >= &DEFECTION_THRESHOLD
}

pub fn to_nearest_move(m: f64) -> f64 {
    if is_cooperation(&m) {
        COOPERATE
    } else {
        DEFECT
    }
}

pub fn to_opposite(m: f64) -> f64 {
    1.0 - m
}
