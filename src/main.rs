mod game;
mod strategies;

use crate::game::*;
use crate::strategies::classic::*;

fn main() {
    let strategies = vec![
        from_functional(unconditional_cooperator),
        from_functional(unconditional_defector),
        from_functional(random),
        from_functional(tit_for_tat),
        from_functional(suspicious_tit_for_tat),
        from_functional(generous_tit_for_tat),
        from_functional(imperfect_tit_for_tat),
        from_functional(tit_for_two_tats),
        from_functional(two_tits_for_tat),
        from_functional(grim),
        from_functional(pavlov),
    ];

    println!("{:?}", strategies.len());
}
