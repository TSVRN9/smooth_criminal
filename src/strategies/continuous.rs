use crate::{from_functional, GameHistory, GameMove, Strategy, COOPERATE, DEFECT, P, R, S, T};

use super::utils;

pub fn all() -> Vec<(&'static str, Box<dyn Strategy>)> {
    vec![
        ("Ambivalent", from_functional(ambivalent)),
        (
            "Ambivalent Suspicious",
            from_functional(ambivalent_suspicious),
        ),
        ("Ambivalent Relaxed", from_functional(ambivalent_relaxed)),
        ("Random", from_functional(random)),
        ("Tit for Tat", from_functional(tit_for_tat)),
        (
            "Suspicious Tit for Tat",
            from_functional(suspicious_tit_for_tat),
        ),
        (
            "Generous Tit for Tat",
            from_functional(generous_tit_for_tat),
        ),
        (
            "Imprecise Tit for Tat",
            from_functional(imprecise_tit_for_tat),
        ),
        ("Tit for Two Tats", from_functional(tit_for_two_tats)),
        ("Two Tits for Tat", from_functional(two_tits_for_tat)),
        ("Grim", from_functional(grim)),
        ("2Pavlov", Box::new(NPavlov::init(2.0))),
        ("4Pavlov", Box::new(NPavlov::init(4.0))),
        ("8Pavlov", Box::new(NPavlov::init(8.0))),
    ]
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

pub fn random(_: &GameHistory) -> f64 {
    rand::random()
}

pub fn tit_for_tat(history: &GameHistory) -> f64 {
    history.last().map_or(COOPERATE, utils::to_opponent_move)
}

pub fn suspicious_tit_for_tat(history: &GameHistory) -> f64 {
    history.last().map_or(DEFECT, utils::to_opponent_move)
}

pub fn generous_tit_for_tat(history: &GameHistory) -> f64 {
    history
        .last()
        .map(utils::to_opponent_move)
        .map_or(DEFECT, |opponent_move| {
            if utils::is_defection(&opponent_move) {
                let g: f64 = (1.0 - ((T - R) / (R - S))).min((R - P) / (T - P));
                if rand::random::<f64>() < g {
                    COOPERATE
                } else {
                    opponent_move
                }
            } else {
                opponent_move
            }
        })
}

pub fn imprecise_tit_for_tat(history: &GameHistory) -> f64 {
    const DELTA: f64 = 0.05;

    history
        .last()
        .map(utils::to_opponent_move)
        .map_or(COOPERATE, |opponent_move| {
            let opponent_move = utils::to_nearest_move(opponent_move);

            opponent_move + ((rand::random::<f64>() * DELTA * 2.0) - DELTA)
        })
}

/// responds with the most cooperative of the last two opponent's moves
pub fn tit_for_two_tats(history: &GameHistory) -> f64 {
    history
        .iter()
        .rev()
        .take(2)
        .map(utils::to_opponent_move)
        .min_by(|a, b| a.total_cmp(b))
        .unwrap_or(COOPERATE)
}

/// responds with the most defective of the last two opponent's moves
pub fn two_tits_for_tat(history: &GameHistory) -> f64 {
    history
        .iter()
        .rev()
        .take(2)
        .map(utils::to_opponent_move)
        .max_by(|a, b| a.total_cmp(b))
        .unwrap_or(COOPERATE)
}

/// responds with the most defective of all opponent's moves
pub fn grim(history: &GameHistory) -> f64 {
    history
        .iter()
        .map(utils::to_opponent_move)
        .max_by(|a, b| a.total_cmp(b))
        .unwrap_or(COOPERATE)
}

#[derive(Debug, Clone)]
pub struct NPavlov {
    n: f64,
    p: f64,
}

impl NPavlov {
    pub fn init(n: f64) -> Self {
        NPavlov { n, p: 1.0 }
    }
}

impl Strategy for NPavlov {
    // unsure if https://plato.stanford.edu/entries/prisoner-dilemma/strategy-table.html has the right implementation?
    fn next_move(&mut self, last_move: Option<GameMove>, _history: &GameHistory) -> f64 {
        self.p += last_move.map_or(COOPERATE, |GameMove(m, o)| {
            match (utils::is_defection(&m), utils::is_defection(&o)) {
                (false, false) | (true, true) => 1.0 / self.n,  // R, P
                (false, true) | (true, false) => -1.0 / self.n, // T, S
            }
        });
        self.p = self.p.clamp(0.0, 1.0);

        1.0 - self.p
    }
}
