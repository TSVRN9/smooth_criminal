/// 0 is cooperate, 1 is defect
pub type Strategy = dyn Fn(&Game, usize) -> f64;
pub type GameResult = [f64; 2];

pub struct Game {
    history: [[f64; 200]; 2]
}

const COOPERATE: i32 = 0;
const DEFECT: i32 = 1;

pub fn play_one_round(first: &Strategy, second: &Strategy, game: &Game, current_round: usize) -> GameResult {
    let defect = [first(game, current_round), second(game, current_round)];
    let cooperation = defect.map(|n| 1.0 - n);
    
    let mut result = [0.0; 2];

    

    return result;
}