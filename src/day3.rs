use itertools::Itertools;

#[aoc_generator(day3, part1)]
pub fn input_generator_part1(input: &str) -> Vec<Rucksack> {
    input.lines().map(parse_rucksack).collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[Rucksack]) -> u64 {
    input.iter().map(|r| r.priority()).sum()
}

#[aoc_generator(day3, part2)]
pub fn input_generator_part2(input: &str) -> Vec<Group> {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            if let Some(tuple) = chunk.into_iter().collect_tuple::<(&str, &str, &str)>() {
                let first = parse_compartment(tuple.0);
                let second = parse_compartment(tuple.1);
                let third = parse_compartment(tuple.2);

                Group {
                    first,
                    second,
                    third,
                }
            } else {
                panic!("Invalid input");
            }
        })
        .collect()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[Group]) -> u64 {
    input.iter().map(|g| g.priority()).sum()
}

pub struct Group {
    first: Compartment,
    second: Compartment,
    third: Compartment,
}

impl Group {
    fn priority(&self) -> u64 {
        let difference: Compartment = self.first & self.second & self.third;
        priority_of(&difference)
    }
}

pub struct Rucksack {
    pub first: Compartment,
    pub second: Compartment,
}

impl Rucksack {
    fn priority(&self) -> u64 {
        let difference: Compartment = self.first & self.second;
        priority_of(&difference)
    }
}

type Compartment = bitmaps::Bitmap<52>;

fn priority_of(s: &Compartment) -> u64 {
    s.into_iter().map(|idx| (idx + 1) as u64).sum::<u64>()
}

fn parse_group(input: &str) -> Group {
    let mut lines = input.lines();
    let first = parse_compartment(lines.next().unwrap());
    let second = parse_compartment(lines.next().unwrap());
    let third = parse_compartment(lines.next().unwrap());

    Group {
        first,
        second,
        third,
    }
}

fn parse_rucksack(input: &str) -> Rucksack {
    let (lhs, rhs) = input.split_at(input.len() / 2);
    Rucksack {
        first: parse_compartment(lhs),
        second: parse_compartment(rhs),
    }
}

fn parse_compartment(s: &str) -> Compartment {
    let mut compartment = Compartment::new();

    for c in s.chars() {
        if ('a'..='z').contains(&c) {
            let bit_idx = (c as u32 - 'a' as u32) as usize;
            compartment.set(bit_idx, true);
        } else if ('A'..='Z').contains(&c) {
            let bit_idx = (c as u32 - 'A' as u32 + 26) as usize;
            compartment.set(bit_idx, true);
        } else {
            panic!("unreachable!")
        }
    }

    compartment
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rucksack_priority() {
        assert_eq!(parse_rucksack("aa").priority(), 1);
        assert_eq!(parse_rucksack("abab").priority(), 3);
        assert_eq!(parse_rucksack("abac").priority(), 1);
        assert_eq!(parse_rucksack("aA").priority(), 0);
        assert_eq!(parse_rucksack("azAZ").priority(), 0);

        assert_eq!(parse_rucksack("vJrwpWtwJgWrhcsFMMfFFhFp").priority(), 16);
        assert_eq!(
            parse_rucksack("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL").priority(),
            38
        );
        assert_eq!(parse_rucksack("PmmdzqPrVvPwwTWBwg").priority(), 42);
        assert_eq!(
            parse_rucksack("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn").priority(),
            22
        );
        assert_eq!(parse_rucksack("ttgJtRGJQctTZtZT").priority(), 20);
        assert_eq!(parse_rucksack("CrZsJsPPZsGzwwsLwLmpwMDw").priority(), 19);
    }

    #[test]
    fn test_parse_compartment() {
        let s = "azAZ";
        let compartment: Compartment = parse_compartment(s);
        assert!(compartment.get(0));
        assert!(compartment.get(25));
        assert!(compartment.get(26));
        assert!(compartment.get(51));
        assert!(!compartment.get(1));
        assert!(!compartment.get(24));
        assert!(!compartment.get(27));
        assert!(!compartment.get(50));
    }
}
