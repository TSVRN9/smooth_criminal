use dyn_clone::DynClone;

pub const COOPERATE: f64 = 0.0;
pub const DEFECT: f64 = 1.0;
pub const NUM_ROUNDS: usize = 1000;

pub const R: f64 = 2.0;
pub const P: f64 = 1.0;
pub const T: f64 = 3.0;
pub const S: f64 = 0.0;

#[derive(Debug, Clone)]
pub struct MatchupResult {
    pub first_name: &'static str,
    pub second_name: &'static str,
    pub overall_result: GameResult,
    pub history: GameHistory,

}
#[derive(Debug, Clone)]
pub struct GameResult(pub f64, pub f64);
#[derive(Debug, Clone)]
pub struct GameMove(pub f64, pub f64);
pub type GameHistory = Vec<GameMove>;

pub trait Strategy: DynClone + Send {
    fn next_move(&mut self, last_move: Option<GameMove>, history: &GameHistory) -> f64;
}

dyn_clone::clone_trait_object!(Strategy);

#[derive(Clone, Debug)]
struct FunctionalStrategyImpl {
    strategy: fn(&GameHistory) -> f64,
}

impl Strategy for FunctionalStrategyImpl {
    fn next_move(&mut self, _last_move: Option<GameMove>, history: &GameHistory) -> f64 {
        (self.strategy)(history)
    }
}

impl GameMove {
    pub fn switch_perspectives(&self) -> GameMove {
        GameMove(self.1, self.0)
    }
}

pub async fn run_competition(
    strategies: Vec<(&'static str, Box<dyn Strategy>)>,
) -> Vec<MatchupResult> {
    let mut tasks = vec![];

    for (first_name, first_strategy) in strategies.iter() {
        for (second_name, second_strategy) in strategies.iter() {
            let first_name = *first_name;
            let second_name = *second_name;
            let mut first_strategy = dyn_clone::clone(&*first_strategy);
            let mut second_strategy = dyn_clone::clone(&*second_strategy);

            let task = tokio::spawn(async move {
                (
                    first_name,
                    second_name,
                    play_strategies(&mut first_strategy, &mut second_strategy),
                )
            });

            tasks.push(task);
        }
    }

    let mut results = vec![];
    for task in tasks {
        if let Ok(result) = task.await {
            results.push(MatchupResult {
                first_name: result.0,
                second_name: result.1,
                overall_result: result.2.0,
                history: result.2.1,
            });
        }
    }

    results
}

pub fn play_strategies(first: &mut Box<dyn Strategy>, second: &mut Box<dyn Strategy>) -> (GameResult, GameHistory) {
    let mut results: GameResult = GameResult(0.0, 0.0);

    let mut history = vec![];
    let mut last_move: Option<GameMove> = None;

    for _ in 0..NUM_ROUNDS {
        let alt_history: GameHistory = history
            .iter()
            .map(|m: &GameMove| m.switch_perspectives())
            .collect();

        let x = first.next_move(last_move.clone(), &history);
        let y = second.next_move(last_move.map(|m| m.switch_perspectives()), &alt_history);

        let result = play_round(x, y);
        let chosen_move = GameMove(x, y);

        results = GameResult(results.0 + result.0, results.1 + result.1);

        history.push(chosen_move.clone());
        last_move = Some(chosen_move);
    }

    return (results, history);
}

pub fn play_round(x: f64, y: f64) -> GameResult {
    return GameResult(eval(x, y), eval(y, x));
}

fn eval(you: f64, other: f64) -> f64 {
    let you = you.clamp(COOPERATE, DEFECT);
    let other = other.clamp(COOPERATE, DEFECT);
    you - (2.0 * other) + 2.0
}

pub fn from_functional(f: fn(&GameHistory) -> f64) -> Box<dyn Strategy> {
    Box::new(FunctionalStrategyImpl { strategy: f })
}