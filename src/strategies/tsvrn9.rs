use std::cmp::Ordering;

use float_cmp::approx_eq;

use crate::{GameHistory, GameMove, Strategy, COOPERATE, DEFECT};

// These are TSVRN9's custom strategies
pub fn all() -> Vec<(&'static str, Box<dyn Strategy>)> {
    vec![
    ]
}

/// This strategy keeps track of its "trust" with other strategies
/// Trust represents how much its move decrements by. (Smaller numbers represent more cooperation)
/// Trust will increment by a 
#[derive(Debug, Clone)]
struct BuildingTrust {
    distrust: f64,
    increment: f64,
}

impl BuildingTrust {
    pub fn init(initial_distrust: f64, relaxation_increment: f64) -> Self {
        BuildingTrust {
            distrust: initial_distrust,
            increment: relaxation_increment,
        }
    }
}

impl Strategy for BuildingTrust {
    fn next_move(&mut self, last_move: Option<GameMove>, _: &GameHistory) -> f64 {
        if let Some(GameMove(_, previous)) = last_move {
            const EXPECTATIONS: f64 = 0.1;

            let disappointment = self.distrust - previous;

            self.distrust = match disappointment.total_cmp(&EXPECTATIONS) {
                Ordering::Greater => 0.0, // more disappointing, betrayed
                _ if approx_eq!(f64, disappointment, 0.0, epsilon = EXPECTATIONS) => self.distrust - self.increment,
                Ordering::Less => previous - self.increment,  // less disappointing, pleasantly surprised
                _ => panic!("unreachable")
            }.clamp(COOPERATE, DEFECT)
        }
        self.distrust
    }
}
