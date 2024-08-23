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
Running this code will generate a csv file with the results of each matchup, and images representing performance. 
Units are in average centi-points per round (cppr), where 300 is equivalent to earning 3 points after every round. 
Red indicates above average performance while blue indicates below average.
White boxes indicate the CPPR

`points.png` displays the cppr of each strategy in a grid, while `win_loss.png` displays the 