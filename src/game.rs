use std::slice::Iter;

pub const COOPERATE: f64 = 0.0;
pub const DEFECT: f64 = 1.0;
pub const NUM_ROUNDS: usize = 200;

pub const R: f64 = 2.0;
pub const P: f64 = 1.0;
pub const T: f64 = 3.0;
pub const S: f64 = 0.0;

#[derive(Debug, Clone)] pub struct GameResult(pub f64, pub f64);
#[derive(Debug, Clone)] pub struct GameMove(pub f64, pub f64);
pub type GameHistory = dyn Iterator<Item = GameMove>;
pub type FunctionalStrategy = dyn Fn(&GameHistory) -> f64;
pub trait Strategy {
    fn next_move(&mut self, last_move: Option<GameMove>, history: GameHistory) -> f64;
}

impl GameMove {
    pub fn switch_perspectives(&self) -> Self {
        GameMove(self.1, self.0)
    }
}

pub fn play_round(x: f64, y: f64) -> GameResult {
    return GameResult(eval(x, y), eval(y, x));
}

pub fn play_strategies(first: &mut dyn Strategy, second: &mut dyn Strategy) -> GameResult {
    let mut results: GameResult = GameResult(0.0, 0.0);

    let mut history_book = vec![];
    let mut last_move: Option<GameMove> = None;

    for _ in 0..NUM_ROUNDS {
        let history = history_book.iter();
        let alt_history = history_book.iter().map(|m: &GameMove| m.switch_perspectives());

        let x = first.next_move(last_move.clone(), history);
        let y = second.next_move(last_move.map(|m| m.switch_perspectives()), alt_history);

        let chosen_move = GameMove(x, y);
        let result = play_round(x, y);

        results = GameResult(results.0 + result.0, results.1 + result.1);

        history_book.push(chosen_move.clone());
        last_move = Some(chosen_move);
    }
    
    return results;
}

pub fn eval(you: f64, other: f64) -> f64 {
    you - (2.0 * other) + 2.0
}