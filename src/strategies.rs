/// Implementations of https://plato.stanford.edu/entries/prisoner-dilemma/strategy-table.html
pub mod classic {
    use crate::{GameHistory, COOPERATE, DEFECT, P, R, S, T};

    use super::util;

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
        history.last().map_or(COOPERATE, util::to_opponent_move)
    }

    pub fn suspicious_tit_for_tat(history: &GameHistory) -> f64 {
        history.last().map_or(DEFECT, util::to_opponent_move)
    }

    pub fn generous_tit_for_tat(history: &GameHistory) -> f64 {
        history
            .last()
            .map(util::to_opponent_move)
            .map_or(DEFECT, |opponent_move| {
                if util::is_defection(&opponent_move) {
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
        history
            .last()
            .map(util::to_opponent_move)
            .map_or(COOPERATE, |opponent_move| {
                const ACCURACY: f64 = 0.95;
                if rand::random::<f64>() < ACCURACY {
                    opponent_move
                } else {
                    1.0 - opponent_move
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
                .map(util::to_opponent_move)
                .filter(util::is_defection)
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
                .map(util::to_opponent_move)
                .any(|m| util::is_defection(&m));

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
            .map(util::to_opponent_move)
            .any(|m| util::is_defection(&m));

        if any_defections { DEFECT } else { COOPERATE }
    }

    pub fn 
}

mod util {
    const DEFECTION_THRESHOLD: f64 = 0.5; // 0.0 is COOPERATE

    use crate::GameMove;

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
}
