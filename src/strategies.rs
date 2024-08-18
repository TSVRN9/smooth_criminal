pub mod classic {
    use crate::{GameHistory, COOPERATE, DEFECT};
    use rand::prelude::*;

    pub fn unconditional_cooperator(_: GameHistory) -> f64 {
        COOPERATE
    }

    pub fn unconditional_defector(_: GameHistory) -> f64 {
        DEFECT
    }

    pub fn 

    pub fn tit_for_tat(history: GameHistory) -> f64 {
        history.last().map_or(0.0, |&(_, opponent_move)| opponent_move)
    }

}
