use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::map_res,
    error::context,
    multi::{many1, separated_list1},
    sequence::terminated,
    IResult,
};

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Vec<u64>> {
    parse_payloads(input).unwrap().1
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[Vec<u64>]) -> u64 {
    input
        .iter()
        .map(|payload| payload.iter().sum())
        .fold(0, |acc, x| if x > acc { x } else { acc })
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[Vec<u64>]) -> u64 {
    input
        .iter()
        .map(|payload| payload.iter().sum::<u64>())
        .sorted()
        .rev()
        .take(3)
        .sum()
}

fn parse_payloads(input: &str) -> IResult<&str, Vec<Vec<u64>>> {
    separated_list1(tag("\n"), payload)(input)
}

fn payload(input: &str) -> IResult<&str, Vec<u64>> {
    context("payload", many1(calories))(input)
}

fn calories(input: &str) -> IResult<&str, u64> {
    context(
        "calories",
        terminated(map_res(digit1, |s: &str| s.parse::<u64>()), tag("\n")),
    )(input)
}
