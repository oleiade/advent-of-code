use camino::Utf8PathBuf;
use id_tree::{InsertBehavior, Node, Tree};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{digit1, space1},
    combinator::{all_consuming, map, map_res, value},
    error::context,
    multi::separated_list1,
    sequence::{pair, preceded, separated_pair, terminated},
    IResult,
};

#[aoc_generator(day7)]
pub fn input_generator_part1(input: &str) -> Vec<LogLine> {
    parse_input(input).unwrap().1
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &[LogLine]) -> color_eyre::Result<usize> {
    let fs = execute_logs(input.to_vec())?;

    // fs is a tree and can be printed using the `write_formatted` method.
    let mut fs_representation = String::new();
    fs.write_formatted(&mut fs_representation)?;
    println!("{fs_representation}");

    let total = fs
        .traverse_pre_order(fs.root_node_id().unwrap())?
        .filter(|node| !node.children().is_empty())
        .map(|node| fs_size_at_node(&fs, node).unwrap())
        .filter(|&size| size <= 100_000)
        .inspect(|size| {
            dbg!(size);
        })
        .sum::<usize>();

    Ok(total)
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &[LogLine]) -> color_eyre::Result<usize> {
    let fs = execute_logs(input.to_vec())?;

    // fs is a tree and can be printed using the `write_formatted` method.
    let mut fs_representation = String::new();
    fs.write_formatted(&mut fs_representation)?;
    println!("{fs_representation}");

    let used = fs_size_at_node(&fs, fs.get(fs.root_node_id().unwrap())?)?;
    let free = 70_000_000_usize - used;

    let smallest = fs
        .traverse_pre_order(fs.root_node_id().unwrap())?
        .filter(|node| !node.children().is_empty())
        .map(|node| fs_size_at_node(&fs, node).unwrap())
        .filter(|&size| size >= 30_000_000 - free)
        .inspect(|size| {
            dbg!(size);
        })
        .min()
        .unwrap();

    Ok(smallest)
}

fn fs_size_at_node(fs: &Tree<FsNode>, node: &Node<FsNode>) -> color_eyre::Result<usize> {
    let mut total = node.data().size;
    for child in node.children() {
        total += fs_size_at_node(fs, fs.get(child)?)?
    }

    Ok(total)
}

fn execute_logs(input: Vec<LogLine>) -> color_eyre::Result<Tree<FsNode>> {
    let mut tree = Tree::<FsNode>::new();

    let root = tree.insert(
        Node::new(FsNode {
            path: "/".into(),
            size: 0,
        }),
        InsertBehavior::AsRoot,
    )?;

    let mut current = root;

    for line in input {
        match line {
            LogLine::Command(c) => match c {
                Command::Ls => {}
                Command::Cd(dir) => match dir.as_str() {
                    "/" => {}
                    ".." => {
                        current = tree.get(&current)?.parent().unwrap().clone();
                    }
                    _ => {
                        let node = FsNode {
                            path: dir.clone(),
                            size: 0,
                        };

                        current =
                            tree.insert(Node::new(node), InsertBehavior::UnderNode(&current))?;
                    }
                },
            },
            LogLine::Entry(entry) => match entry {
                Entry::Directory(dir) => {}
                Entry::File(size, path) => {
                    let node = FsNode {
                        path: path.clone(),
                        size,
                    };

                    tree.insert(Node::new(node), InsertBehavior::UnderNode(&current))?;
                }
            },
        }
    }

    Ok(tree)
}

#[derive(Clone, Debug, Default)]
pub struct FsNode {
    path: Utf8PathBuf,
    size: usize,
}

fn parse_input(input: &str) -> IResult<&str, Vec<LogLine>> {
    all_consuming(separated_list1(tag("\n"), parse_log_line))(input)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LogLine {
    Command(Command),
    Entry(Entry),
}

fn parse_log_line(input: &str) -> IResult<&str, LogLine> {
    context(
        "log_line",
        alt((map(command, LogLine::Command), map(entry, LogLine::Entry))),
    )(input)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Command {
    Cd(Utf8PathBuf),
    Ls,
}

fn command(input: &str) -> IResult<&str, Command> {
    context(
        "command",
        preceded(terminated(tag("$"), space1), alt((cd, ls))),
    )(input)
}

fn cd(input: &str) -> IResult<&str, Command> {
    context(
        "cd",
        map(preceded(pair(tag("cd"), space1), path), Command::Cd),
    )(input)
}

fn ls(input: &str) -> IResult<&str, Command> {
    context("ls", value(Command::Ls, tag("ls")))(input)
}

fn path(input: &str) -> IResult<&str, Utf8PathBuf> {
    context(
        "path",
        map(
            take_while1(|c: char| c.is_alphanumeric() || c == '/' || c == '.'),
            Into::into,
        ),
    )(input)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Entry {
    File(usize, Utf8PathBuf),
    Directory(Utf8PathBuf),
}

fn entry(input: &str) -> IResult<&str, Entry> {
    context("entry", alt((file_entry, dir_entry)))(input)
}

fn dir_entry(input: &str) -> IResult<&str, Entry> {
    context(
        "dir_entry",
        map(preceded(pair(tag("dir"), space1), path), |path| {
            Entry::Directory(path)
        }),
    )(input)
}

fn file_entry(input: &str) -> IResult<&str, Entry> {
    context(
        "file",
        map(
            separated_pair(unsigned_size, space1, path),
            |(size, path)| Entry::File(size, path),
        ),
    )(input)
}

fn unsigned_size(input: &str) -> IResult<&str, usize> {
    context(
        "unsigned_size",
        map_res(digit1, |s: &str| s.parse::<usize>()),
    )(input)
}
