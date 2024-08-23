# Smooth Criminal
This is an implementation of [Nicky Case's Smooth Criminal](https://blog.ncase.me/backlog/#project_7). 
These are the point values for decisions each strategy can make. Each strategy plays every other strategy for 2000 rounds.

| A/B          |   A Cooperates   |    A Defects   |
|--------------|------------------|----------------|
| B Cooperates |    2.0/2.0       |    3.0/0.0     |
| B Defects    |    0.0/3.0       |    1.0/1.0     |

The twist here is that strategies can opt for values between cooperating and defecting. 
`0.0` represents cooperation while `1.0` represents defection.
Point values are determined through linear interpolation.
Below is the eval function representing the point calcuation.

```rust
fn eval(you: f64, other: f64) -> f64 {
    let you = you.clamp(COOPERATE, DEFECT);
    let other = other.clamp(COOPERATE, DEFECT);
    you - (2.0 * other) + 2.0
}
```

## Usage
Running this code with `cargo run --release` will generate a csv file with the results of each matchup, and images representing performance. 
It is recommended to build the optimized binary, as the increase in build time drastically increases performance.

Units are in average centi-points per round (cppr), where 300 is equivalent to earning 3 points after every round. 
Red indicates above average performance while blue indicates below average.
Columns are left unlabelled, but follow the same order as the rows from left to right.
White outlines indicate that the strategy is performing against itself, but can also be used as guidelines.

`points.png` displays the cppr of each strategy in a grid, where blue cells represent above average cppr while red cells represent below average cppr.
`win_loss.png` displays the difference in cppr between two strategies, where blue cells represent positive cpprs while red represents negative cpprs.

![Example of `points.png`, with labels to the left and a colored grid to the right](./assets/points.png)
> Example of `points.png`

## Roadmap
Currently there is no way to view individual matchups and the results of each round. 
The next iteration will feature an interactive UI to inspect individual matchups, exclude/include strategies, and provide an easy way to import and export data.