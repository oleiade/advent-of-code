use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, space1},
    combinator::{map, map_res, opt, value},
    error::context,
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult,
};

#[aoc_generator(day5)]
pub fn input_generator_part1(input: &str) -> (Storage, Vec<Instruction>) {
    parse(input).unwrap().1
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &(Storage, Vec<Instruction>)) -> String {
    let game = (*input).clone();
    let mut storage = game.0;
    let instructions = game.1;

    for instruction in instructions.iter() {
        for _ in 0..instruction.quantity {
            if let Some(v) = storage.stacks[instruction.from - 1].pop() {
                storage.stacks[instruction.to - 1].push(v)
            }
        }
    }

    storage
        .stacks
        .iter()
        .filter_map(|stack| stack.last())
        .join("")
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &(Storage, Vec<Instruction>)) -> String {
    let game = (*input).clone();
    let mut storage = game.0;
    let instructions = game.1;

    for instruction in instructions.iter() {
        let from_size = storage.stacks[instruction.from - 1].len();
        let to_size = storage.stacks[instruction.to - 1].len();

        let values: Vec<String> = storage.stacks[instruction.from - 1]
            .drain(from_size - instruction.quantity as usize..)
            .collect();

        values
            .iter()
            .for_each(|v| storage.stacks[instruction.to - 1].push(v.to_string()));
    }

    storage
        .stacks
        .iter()
        .filter_map(|stack| stack.last())
        .join("")
}

#[derive(Clone, Debug)]
pub struct Storage {
    stacks: Vec<Stack>,
}

type Stack = Vec<Crate>;
type Crate = String;

#[derive(Clone, Debug)]
pub struct Instruction {
    action: Action,
    quantity: u64,
    from: usize,
    to: usize,
}

#[derive(Clone, Debug)]
pub enum Action {
    Move,
}

// FIXME: For some reason I don't have time to look into, the parser
// doesn't parse the last line, so one needs to add a dummy one at the end
// of the file.
fn parse(input: &str) -> IResult<&str, (Storage, Vec<Instruction>)> {
    context(
        "parser",
        terminated(
            separated_pair(storage, tag("\n"), many1(instruction)),
            opt(tag("\n")),
        ),
    )(input)
}

fn storage(input: &str) -> IResult<&str, Storage> {
    let (remain, store) = terminated(separated_list1(tag("\n"), storage_line), tag("\n"))(input)?;
    let (remain, indices) = delimited(
        tag(" "),
        separated_list1(tag("   "), unsigned_size),
        tag(" \n"),
    )(remain)?;

    let mut storage = Storage {
        stacks: vec![vec![]; indices.len()],
    };
    for row in store.iter().rev() {
        for (idx, value) in row.iter().enumerate() {
            if let Some(v) = value {
                storage.stacks[idx].push(v.to_string())
            }
        }
    }

    Ok((remain, storage))
}

fn storage_line(input: &str) -> IResult<&str, Vec<Option<Crate>>> {
    context(
        "storage_line",
        separated_list1(
            tag(" "),
            alt((map(tag("   "), |t| None), map(parse_crate, Some))),
        ),
    )(input)
}

fn parse_crate(input: &str) -> IResult<&str, Crate> {
    context(
        "crate",
        map(delimited(tag("["), alpha1, tag("]")), String::from),
    )(input)
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    context(
        "instruction",
        map(
            tuple((
                separated_pair(action, space1, unsigned_integer_64),
                preceded(delimited(space1, tag("from"), space1), unsigned_size),
                delimited(
                    delimited(space1, tag("to"), space1),
                    unsigned_size,
                    tag("\n"),
                ),
            )),
            |((action, quantity), from, to)| Instruction {
                action,
                quantity,
                from,
                to,
            },
        ),
    )(input)
}

fn action(input: &str) -> IResult<&str, Action> {
    context("action", value(Action::Move, tag("move")))(input)
}

fn unsigned_size(input: &str) -> IResult<&str, usize> {
    context(
        "unsigned_size",
        map_res(digit1, |s: &str| s.parse::<usize>()),
    )(input)
}

fn unsigned_integer_64(input: &str) -> IResult<&str, u64> {
    context(
        "unsigned_integer_64",
        map_res(digit1, |s: &str| s.parse::<u64>()),
    )(input)
}
