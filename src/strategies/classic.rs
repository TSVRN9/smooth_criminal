// Implementations of https://plato.stanford.edu/entries/prisoner-dilemma/strategy-table.html

use crate::{from_functional, GameHistory, GameMove, Strategy, COOPERATE, DEFECT, P, R, S, T};

use super::utils;

pub fn all() -> Vec<(&'static str, Box<dyn Strategy>)> {
    vec![
        (
            "Classic Unconditional Cooperator",
            from_functional(unconditional_cooperator),
        ),
        (
            "Classic Unconditional Defector",
            from_functional(unconditional_defector),
        ),
        ("Classic Random", from_functional(random)),
        ("Classic Tit for Tat", from_functional(tit_for_tat)),
        (
            "Classic Suspicious Tit for Tat",
            from_functional(suspicious_tit_for_tat),
        ),
        (
            "Classic Generous Tit for Tat",
            from_functional(generous_tit_for_tat),
        ),
        (
            "Classic Imperfect Tit for Tat",
            from_functional(imperfect_tit_for_tat),
        ),
        (
            "Classic Tit for Two Tats",
            from_functional(tit_for_two_tats),
        ),
        (
            "Classic Two Tits for Tat",
            from_functional(two_tits_for_tat),
        ),
        ("Classic Grim", from_functional(grim)),
        ("Classic Pavlov", from_functional(pavlov)),
        ("Classic 2Pavlov", Box::new(NPavlov::init(2.0))),
        ("Classic 4Pavlov", Box::new(NPavlov::init(4.0))),
        ("Classic 8Pavlov", Box::new(NPavlov::init(8.0))),
    ]
}

pub fn unconditional_cooperator(_: &GameHistory) -> f64 {
    COOPERATE
}

pub fn unconditional_defector(_: &GameHistory) -> f64 {
    DEFECT
}

pub fn random(_: &GameHistory) -> f64 {
    if rand::random::<bool>() {
        COOPERATE
    } else {
        DEFECT
    }
}

// skip p_cooperator

pub fn tit_for_tat(history: &GameHistory) -> f64 {
    history
        .last()
        .map(utils::to_opponent_move)
        .map_or(COOPERATE, utils::to_nearest_move)
}

pub fn suspicious_tit_for_tat(history: &GameHistory) -> f64 {
    history
        .last()
        .map(utils::to_opponent_move)
        .map_or(DEFECT, utils::to_nearest_move)
}

pub fn generous_tit_for_tat(history: &GameHistory) -> f64 {
    history
        .last()
        .map(utils::to_opponent_move)
        .map_or(DEFECT, |opponent_move| {
            let opponent_move = utils::to_nearest_move(opponent_move);

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

// skipping gradual tit for tat
//    pub fn gradual_tit_for_tat(history: GameHistory) -> f64 {
//        if history.is_empty() {
//            return COOPERATE;
//        }
//
//        let opponent_defections = history
//            .iter()
//            .map(|&(_, opponent_move)| opponent_move)
//            .filter(|m| m < &DEFECTION_THRESHOLD)
//            .count();
//
//        let last_string = history
//            .iter()
//            .rev()
//            .map(|&(my_move, _)| my_move)
//            .skip_while(|m| m > &DEFECTION_THRESHOLD)
//            .take_while(|m| m < &DEFECTION_THRESHOLD)
//            .count();
//
//        if opponent_defections <= last_string {
//            cont...
//        }
//    }

pub fn imperfect_tit_for_tat(history: &GameHistory) -> f64 {
    const ACCURACY: f64 = 0.95;

    history
        .last()
        .map(utils::to_opponent_move)
        .map_or(COOPERATE, |opponent_move| {
            let opponent_move = utils::to_nearest_move(opponent_move);

            if rand::random::<f64>() < ACCURACY {
                opponent_move
            } else {
                utils::to_opposite(opponent_move)
            }
        })
}

pub fn tit_for_two_tats(history: &GameHistory) -> f64 {
    if history.len() <= 2 {
        COOPERATE
    } else {
        let two_defections = history
            .iter()
            .rev()
            .take(2)
            .map(utils::to_opponent_move)
            .filter(utils::is_defection)
            .count()
            == 2;

        if two_defections {
            DEFECT
        } else {
            COOPERATE
        }
    }
}

pub fn two_tits_for_tat(history: &GameHistory) -> f64 {
    if history.len() <= 2 {
        COOPERATE
    } else {
        let any_defections = history
            .iter()
            .rev()
            .take(2)
            .map(utils::to_opponent_move)
            .any(|m| utils::is_defection(&m));

        if any_defections {
            DEFECT
        } else {
            COOPERATE
        }
    }
}

// skipped omega_tit_for_tat

pub fn grim(history: &GameHistory) -> f64 {
    let any_defections = history
        .iter()
        .map(utils::to_opponent_move)
        .any(|m| utils::is_defection(&m));

    if any_defections {
        DEFECT
    } else {
        COOPERATE
    }
}

pub fn pavlov(history: &GameHistory) -> f64 {
    history.last().map_or(COOPERATE, |GameMove(m, o)| {
        match (utils::is_defection(m), utils::is_defection(o)) {
            (false, false) | (true, false) => *m,                   // R, T
            (false, true) | (true, true) => utils::to_opposite(*m), // P, S
        }
    })
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

        if self.p < rand::random() {
            COOPERATE
        } else {
            DEFECT
        }
    }
}
