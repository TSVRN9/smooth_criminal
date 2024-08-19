mod game;
mod strategies;

use crate::game::*;
use crate::strategies::classic::*;
use rayon::prelude::*;

fn main() {
    let strategies = [
        (
            "Classic Unconditional Cooperator",
            from_functional(unconditional_cooperator),
        ),
        (
            "Classic Unconditional Defector",
            from_functional(unconditional_defector),
        ),
        ("Classic Random", from_functional(random)),
        ("Classic Tit for Tat", from_functional(tit_for_tat)),
        (
            "Classic Suspicious Tit for Tat",
            from_functional(suspicious_tit_for_tat),
        ),
        (
            "Classic Generous Tit for Tat",
            from_functional(generous_tit_for_tat),
        ),
        (
            "Classic Imperfect Tit for Tat",
            from_functional(imperfect_tit_for_tat),
        ),
        (
            "Classic Tit for Two Tats",
            from_functional(tit_for_two_tats),
        ),
        (
            "Classic Two Tits for Tat",
            from_functional(two_tits_for_tat),
        ),
        ("Classic Grim", from_functional(grim)),
        ("Classic Pavlov", from_functional(pavlov)),
        ("Classic 4Pavlov", Box::new(NPavlov::init()))
    ];

    for (first_name, first_strategy) in strategies.iter() {
        let (wins, ties, losses, points) = strategies.iter().map(|(second_name, second_strategy)| {
            let mut first_strategy = first_strategy.clone();
            let mut second_strategy = second_strategy.clone();

            let GameResult(first_score, second_score) =
                play_strategies(&mut first_strategy, &mut second_strategy);

            println!("{first_name} - {first_score} vs. {second_score} - {second_name}");

            match first_score.total_cmp(&second_score) {
                std::cmp::Ordering::Greater => (1, 0, 0, first_score),
                std::cmp::Ordering::Equal => (0, 1, 0, first_score),
                std::cmp::Ordering::Less => (0, 0, 1, first_score),
            }
        }).reduce(|a, b| (a.0 + b.0, a.1 + b.1, a.2 + b.2, a.3 + b.3)).expect("0 Games played??");

        println!("--------------{first_name} with {points} points, {wins} wins, {ties} ties, {losses} losses------------------");
    }
}
