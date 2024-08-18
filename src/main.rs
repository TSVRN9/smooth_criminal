mod game;
mod strategies;

use crate::game::*;
use crate::strategies::tit_for_tat;

fn main() {
    tit_for_tat();
    println!("{:?}", play_round(DEFECT, DEFECT));
}
