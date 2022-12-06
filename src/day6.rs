use itertools::Itertools;

#[aoc_generator(day6)]
pub fn input_generator_part1(input: &str) -> Vec<char> {
    include_str!("../input/2022/day6.txt").chars().collect_vec()
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &[char]) -> u64 {
    input
        .windows(4)
        .into_iter()
        .position(|w| w.iter().unique().collect_vec().len() == w.len())
        .map(|pos| pos as u64 + 4)
        .expect("should have found a message")
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &[char]) -> u64 {
    input
        .windows(14)
        .into_iter()
        .position(|w| w.iter().unique().collect_vec().len() == w.len())
        .map(|pos| pos as u64 + 14)
        .expect("should have found a message")
}
