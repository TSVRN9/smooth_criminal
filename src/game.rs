pub const COOPERATE: f64 = 0.0;
pub const DEFECT: f64 = 1.0;
pub const NUM_ROUNDS: usize = 200;

pub const R: f64 = 2.0;
pub const P: f64 = 1.0;
pub const T: f64 = 3.0;
pub const S: f64 = 0.0;

pub type GameResult = (f64, f64);
pub type GameHistory = Vec<(f64, f64)>;
pub type Strategy = dyn Fn(&GameHistory) -> f64;


pub fn play_round(x: f64, y: f64) -> GameResult {
    return (eval(x, y), eval(y, x));
}

pub fn play_strategies(first: &Strategy, second: &Strategy) -> GameResult {
    let mut history: GameHistory = vec![];
    let mut results: GameResult = (0.0, 0.0);

    for _ in 0..NUM_ROUNDS {
        let alt_history = &history.iter().map(|(a, b)| (*b, *a)).collect();

        let x = first(&history);
        let y = second(&alt_history);

        let result = play_round(x, y);
        results = (results.0 + result.0, results.1 + result.1);
        history.push((x, y));
    }
    
    return results;
}

pub fn eval(you: f64, other: f64) -> f64 {
    you - (2.0 * other) + 2.0
}