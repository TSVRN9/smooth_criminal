use crate::{GameHistory, GameMove, Strategy, DEFECT};

// These are TSVRN9's custom strategies
pub fn all() -> Vec<(&'static str, Box<dyn Strategy>)> {
    vec![
        ("Detente", Box::new(Detente::init(1.0, 0.1))),
        ("Suspicious Detente", Box::new(Detente::init(0.0, 0.1))),
        ("Trusting Detente", Box::new(Detente::init(1.0, 0.5)))
    ]
}

/// This strategy keeps track of its "comfort" with other strategies
/// Comfort is how much each move decrements by. (Smaller numbers represent more cooperation, so as comfort increases, the f64 returns decreases)
/// Trust is how much comfort increments by, which is constant
/// Defections will reset comfort and lead to the next move being a defection
/// Cooperation will decrement Detente's previous move by comfort
/// A move is considered cooperative if it is less than max(1 - comfort / 2, 0.1)
#[derive(Debug, Clone)]
struct Detente {
    comfort: f64,
    trust: f64,
}

impl Detente {
    pub fn init(intial_comfort: f64, trust: f64) -> Self {
        Detente {
            comfort: intial_comfort,
            trust,
        }
    }
}

impl Strategy for Detente {
    fn next_move(&mut self, last_move: Option<GameMove>, _: &GameHistory) -> f64 {
        if let Some(GameMove(_, previous)) = last_move {
            let is_cooperative = previous < (1.0 - (self.comfort / 2.0)).max(0.1);

            let next_move = if is_cooperative {
                let v = previous - self.comfort;
                self.comfort += self.trust;
                v
            } else {
                self.comfort = 0.0;
                DEFECT
            };

            next_move
        } else {
            1.0 - self.comfort
        }
    }
}
