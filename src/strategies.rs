/// Implementations of https://plato.stanford.edu/entries/prisoner-dilemma/strategy-table.html
pub mod classic {
    const DEFECTION_THRESHOLD: f64 = 0.5; // 0.0 is COOPERATE

    use crate::{GameHistory, COOPERATE, DEFECT, P, R, S, T};

    pub fn unconditional_cooperator(_: GameHistory) -> f64 {
        COOPERATE
    }

    pub fn unconditional_defector(_: GameHistory) -> f64 {
        DEFECT
    }

    pub fn random(_: GameHistory) -> f64 {
        if rand::random::<bool>() {
            COOPERATE
        } else {
            DEFECT
        }
    }

    // skip p_cooperator

    pub fn tit_for_tat(history: GameHistory) -> f64 {
        history
            .last()
            .map_or(COOPERATE, |&(_, opponent_move)| opponent_move)
    }

    pub fn suspicious_tit_for_tat(history: GameHistory) -> f64 {
        history
            .last()
            .map_or(DEFECT, |&(_, opponent_move)| opponent_move)
    }

    pub fn generous_tit_for_tat(history: GameHistory) -> f64 {
        history.last().map_or(DEFECT, |&(_, opponent_move)| {
            if opponent_move < DEFECTION_THRESHOLD {
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

    pub fn imperfect_tit_for_tat(history: GameHistory) -> f64 {
        history.last().map_or(COOPERATE, |&(_, opponent_move)| {
            const ACCURACY: f64 = 0.95;
            if rand::random::<f64>() < ACCURACY {
                opponent_move
            } else {
                1.0 - opponent_move
            }
        })
    }

    pub fn tit_for_two_tats(history: GameHistory) -> f64
}
