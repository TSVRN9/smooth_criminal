mod game;
mod strategies;

use crate::game::*;
use crate::strategies::classic::tit_for_tat;

fn main() {
    println!("{:?}", play_round(DEFECT, DEFECT));
}
