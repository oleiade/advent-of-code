use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, map_res},
    error::context,
    sequence::separated_pair,
    IResult,
};
use std::cmp;

#[aoc_generator(day4)]
pub fn input_generator_part1(input: &str) -> Vec<(Range, Range)> {
    input
        .lines()
        .map(|line| range_pair(line).unwrap().1)
        .collect()
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[(Range, Range)]) -> u64 {
    input
        .iter()
        .map(|(lhs, rhs)| {
            u64::from(
                ((lhs.min <= rhs.min) && (lhs.max >= rhs.max))
                    || ((rhs.min <= lhs.min) && (rhs.max >= lhs.max)),
            )
        })
        .sum()
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[(Range, Range)]) -> u64 {
    input
        .iter()
        .map(|(lhs, rhs)| match overlapps(lhs, rhs) {
            true => 1,
            false => 0,
        })
        .sum()
}

fn overlapps(lhs: &Range, rhs: &Range) -> bool {
    cmp::max(lhs.min, rhs.min) <= cmp::min(lhs.max, rhs.max)
}

#[derive(Clone, Copy)]
pub struct Range {
    pub min: u64,
    pub max: u64,
}

fn range_pair(input: &str) -> IResult<&str, (Range, Range)> {
    context(
        "range_pair",
        map(separated_pair(range, tag(","), range), |(min, max)| {
            (min, max)
        }),
    )(input)
}

fn range(input: &str) -> IResult<&str, Range> {
    context(
        "range",
        map(
            separated_pair(unsigned_integer, tag("-"), unsigned_integer),
            |(min, max): (u64, u64)| Range { min, max },
        ),
    )(input)
}

fn unsigned_integer(input: &str) -> IResult<&str, u64> {
    context(
        "unsigned_integer",
        map_res(digit1, |s: &str| s.parse::<u64>()),
    )(input)
}
