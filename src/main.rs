mod game;
mod strategies {
    pub mod classic;
    pub mod continuous;
    pub mod utils;
}

use strategies::{classic, continuous};

use crate::game::*;

fn main() {
    let strategies: Vec<(&'static str, Box<dyn Strategy>)> = vec![
        classic::all(),
        continuous::all(),
    ].into_iter().flatten().collect();

    for (first_name, first_strategy) in strategies.iter() {
        let (wins, ties, losses, points) = strategies
            .iter()
            .map(|(second_name, second_strategy)| {
                let mut first_strategy = dyn_clone::clone(&*first_strategy);
                let mut second_strategy = dyn_clone::clone(&*second_strategy);

                let GameResult(first_score, second_score) =
                    play_strategies(&mut first_strategy, &mut second_strategy);

                println!("{first_name} - {first_score} vs. {second_score} - {second_name}");

                match first_score.total_cmp(&second_score) {
                    std::cmp::Ordering::Greater => (1, 0, 0, first_score),
                    std::cmp::Ordering::Equal => (0, 1, 0, first_score),
                    std::cmp::Ordering::Less => (0, 0, 1, first_score),
                }
            })
            .reduce(|a, b| (a.0 + b.0, a.1 + b.1, a.2 + b.2, a.3 + b.3))
            .expect("0 Games played??");

        println!("--------------{first_name} with {points} points, {wins} wins, {ties} ties, {losses} losses------------------");
    }
}
