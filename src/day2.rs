use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::multispace1,
    combinator::{map, value},
    error::context,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[aoc_generator(day2, part1)]
pub fn input_generator_part1(input: &str) -> Vec<Round> {
    separated_list1(tag("\n"), round)(input).unwrap().1
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Round]) -> i64 {
    input.iter().map(|round| round.score()).sum()
}

#[aoc_generator(day2, part2)]
pub fn input_generator_part2(input: &str) -> Vec<StrategizedRound> {
    separated_list1(tag("\n"), strategized_round)(input)
        .unwrap()
        .1
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[StrategizedRound]) -> i64 {
    input.iter().map(|round| round.score()).sum()
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Pick {
    Rock,
    Paper,
    Scissor,
}

impl Pick {
    fn score(&self) -> i64 {
        match self {
            Pick::Rock => 1,
            Pick::Paper => 2,
            Pick::Scissor => 3,
        }
    }

    /// strategic_counterpart returns the pick that opposes the opponent's pick
    /// and fits the selected strategy.
    fn strategic_counterpart(&self, strategy: Strategy) -> Pick {
        match strategy {
            Strategy::Win => match self {
                Pick::Rock => Pick::Paper,
                Pick::Paper => Pick::Scissor,
                Pick::Scissor => Pick::Rock,
            },
            Strategy::Lose => match self {
                Pick::Rock => Pick::Scissor,
                Pick::Paper => Pick::Rock,
                Pick::Scissor => Pick::Paper,
            },
            Strategy::Draw => match self {
                Pick::Rock => Pick::Rock,
                Pick::Paper => Pick::Paper,
                Pick::Scissor => Pick::Scissor,
            },
        }
    }
}

/// Round of the game
#[derive(Debug, PartialEq, Eq)]
pub struct Round {
    pub them: Pick,
    pub us: Pick,
}

impl Round {
    /// outcome returns the ouctome of a round
    fn outcome(&self) -> Outcome {
        match self.us.score() - self.them.score() {
            0 => Outcome::Draw,
            1 | -2 => Outcome::Win,
            -1 | 2 => Outcome::Lose,
            _ => panic!("Invalid score difference"),
        }
    }

    /// score returns the score of a round based on its outcome
    fn score(&self) -> i64 {
        match self.outcome() {
            Outcome::Win => self.us.score() + 6,
            Outcome::Draw => self.us.score() + 3,
            Outcome::Lose => self.us.score(),
        }
    }
}

/// StrategizedRound represents a round where we have a strategy for how to play
/// against our opponent available to us.
pub struct StrategizedRound {
    pub them: Pick,
    pub strategy: Strategy,
}

impl StrategizedRound {
    /// score computes the score obtained by playing the strategy against the
    /// opponent's pick.
    fn score(&self) -> i64 {
        let pick = self.them.strategic_counterpart(self.strategy);
        let round = Round {
            them: self.them.clone(),
            us: pick,
        };
        round.score()
    }
}

/// Outcome represents the outcome of a round
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    Win,
    Lose,
    Draw,
}

/// Strategy represents the strategy to use against
/// the opponent, as obtained from the elf's input
type Strategy = Outcome;

fn strategized_round(input: &str) -> IResult<&str, StrategizedRound> {
    context(
        "strategized_round",
        map(
            separated_pair(pick, multispace1, strategy),
            |(them, strategy)| StrategizedRound { them, strategy },
        ),
    )(input)
}

fn round(input: &str) -> IResult<&str, Round> {
    context(
        "round",
        map(separated_pair(pick, multispace1, pick), |(them, us)| {
            Round { them, us }
        }),
    )(input)
}

fn pick(input: &str) -> IResult<&str, Pick> {
    context(
        "pick",
        alt((
            value(Pick::Rock, tag("A")),
            value(Pick::Paper, tag("B")),
            value(Pick::Scissor, tag("C")),
            value(Pick::Rock, tag("X")),
            value(Pick::Paper, tag("Y")),
            value(Pick::Scissor, tag("Z")),
        )),
    )(input)
}

fn strategy(input: &str) -> IResult<&str, Strategy> {
    context(
        "strategy",
        alt((
            value(Strategy::Lose, tag("X")),
            value(Strategy::Draw, tag("Y")),
            value(Strategy::Win, tag("Z")),
        )),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pick_score() {
        assert_eq!(Pick::Rock.score(), 1);
        assert_eq!(Pick::Paper.score(), 2);
        assert_eq!(Pick::Scissor.score(), 3);
    }

    #[test]
    fn test_outcome() {
        assert_eq!(
            Round {
                them: Pick::Rock,
                us: Pick::Rock
            }
            .outcome(),
            Outcome::Draw
        );

        assert_eq!(
            Round {
                them: Pick::Rock,
                us: Pick::Paper
            }
            .outcome(),
            Outcome::Win
        );

        assert_eq!(
            Round {
                them: Pick::Rock,
                us: Pick::Scissor
            }
            .outcome(),
            Outcome::Lose
        );

        assert_eq!(
            Round {
                them: Pick::Paper,
                us: Pick::Rock
            }
            .outcome(),
            Outcome::Lose
        );

        assert_eq!(
            Round {
                them: Pick::Paper,
                us: Pick::Paper
            }
            .outcome(),
            Outcome::Draw
        );

        assert_eq!(
            Round {
                them: Pick::Paper,
                us: Pick::Scissor
            }
            .outcome(),
            Outcome::Win
        );

        assert_eq!(
            Round {
                them: Pick::Scissor,
                us: Pick::Rock
            }
            .outcome(),
            Outcome::Win
        );

        assert_eq!(
            Round {
                them: Pick::Scissor,
                us: Pick::Paper
            }
            .outcome(),
            Outcome::Lose
        );

        assert_eq!(
            Round {
                them: Pick::Scissor,
                us: Pick::Scissor
            }
            .outcome(),
            Outcome::Draw
        );
    }

    #[test]
    fn test_round_score() {
        assert_eq!(
            Round {
                them: Pick::Rock,
                us: Pick::Rock
            }
            .score(),
            4
        );

        assert_eq!(
            Round {
                them: Pick::Rock,
                us: Pick::Paper
            }
            .score(),
            8
        );

        assert_eq!(
            Round {
                them: Pick::Rock,
                us: Pick::Scissor
            }
            .score(),
            3
        );

        assert_eq!(
            Round {
                them: Pick::Paper,
                us: Pick::Rock
            }
            .score(),
            1
        );

        assert_eq!(
            Round {
                them: Pick::Paper,
                us: Pick::Paper
            }
            .score(),
            5
        );

        assert_eq!(
            Round {
                them: Pick::Paper,
                us: Pick::Scissor
            }
            .score(),
            9
        );

        assert_eq!(
            Round {
                them: Pick::Scissor,
                us: Pick::Rock
            }
            .score(),
            7
        );

        assert_eq!(
            Round {
                them: Pick::Scissor,
                us: Pick::Paper
            }
            .score(),
            2
        );

        assert_eq!(
            Round {
                them: Pick::Scissor,
                us: Pick::Scissor
            }
            .score(),
            6
        );
    }

    #[test]
    fn test_game() {
        let input = "A A\nA B\nA C\nB A\nB B\nB C\nC A\nC B\nC C\n";
        let game = parse_game(input).unwrap().1;
        assert_eq!(game.len(), 9);
        assert_eq!(
            game[0],
            Round {
                them: Pick::Rock,
                us: Pick::Rock
            }
        );
        assert_eq!(
            game[1],
            Round {
                them: Pick::Rock,
                us: Pick::Paper
            }
        );
        assert_eq!(
            game[2],
            Round {
                them: Pick::Rock,
                us: Pick::Scissor
            }
        );
        assert_eq!(
            game[3],
            Round {
                them: Pick::Paper,
                us: Pick::Rock
            }
        );
        assert_eq!(
            game[4],
            Round {
                them: Pick::Paper,
                us: Pick::Paper
            }
        );
        assert_eq!(
            game[5],
            Round {
                them: Pick::Paper,
                us: Pick::Scissor
            }
        );
        assert_eq!(
            game[6],
            Round {
                them: Pick::Scissor,
                us: Pick::Rock
            }
        );
        assert_eq!(
            game[7],
            Round {
                them: Pick::Scissor,
                us: Pick::Paper
            }
        );
        assert_eq!(
            game[8],
            Round {
                them: Pick::Scissor,
                us: Pick::Scissor
            }
        );
    }

    #[test]
    fn test_round() {
        assert_eq!(
            round("A A"),
            Ok((
                "",
                Round {
                    them: Pick::Rock,
                    us: Pick::Rock
                }
            ))
        );

        assert_eq!(
            round("A B"),
            Ok((
                "",
                Round {
                    them: Pick::Rock,
                    us: Pick::Paper
                }
            ))
        );

        assert_eq!(
            round("A C"),
            Ok((
                "",
                Round {
                    them: Pick::Rock,
                    us: Pick::Scissor
                }
            ))
        );

        assert_eq!(
            round("B A"),
            Ok((
                "",
                Round {
                    them: Pick::Paper,
                    us: Pick::Rock
                }
            ))
        );

        assert_eq!(
            round("B B"),
            Ok((
                "",
                Round {
                    them: Pick::Paper,
                    us: Pick::Paper
                }
            ))
        );

        assert_eq!(
            round("B C"),
            Ok((
                "",
                Round {
                    them: Pick::Paper,
                    us: Pick::Scissor
                }
            ))
        );

        assert_eq!(
            round("C A"),
            Ok((
                "",
                Round {
                    them: Pick::Scissor,
                    us: Pick::Rock
                }
            ))
        );

        assert_eq!(
            round("C B"),
            Ok((
                "",
                Round {
                    them: Pick::Scissor,
                    us: Pick::Paper
                }
            ))
        );

        assert_eq!(
            round("C C"),
            Ok((
                "",
                Round {
                    them: Pick::Scissor,
                    us: Pick::Scissor
                }
            ))
        );
    }

    #[test]
    fn test_pick() {
        assert_eq!(pick("A"), Ok(("", Pick::Rock)));
        assert_eq!(pick("B"), Ok(("", Pick::Paper)));
        assert_eq!(pick("C"), Ok(("", Pick::Scissor)));
        assert_eq!(pick("X"), Ok(("", Pick::Rock)));
        assert_eq!(pick("Y"), Ok(("", Pick::Paper)));
        assert_eq!(pick("Z"), Ok(("", Pick::Scissor)));
    }
}
