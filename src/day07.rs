use std::collections::{HashMap, HashSet};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{alpha1, multispace1, newline},
    combinator::map,
    sequence::{delimited, pair, separated_pair, terminated},
    IResult,
};

#[derive(Debug)]
enum CommandLineIO {
    CD(String),
    LS,
    Directory(String),
    File(u64, String),
}

#[allow(dead_code)]
fn part_1(input: &str) -> u64 {
    let commands = Commands { inner: input };
    let directory_sizes = all_directory_sizes(commands);

    directory_sizes
        .values()
        .filter(|value| **value <= 100000)
        .sum()
}

#[allow(dead_code)]
fn part_2(input: &str) -> u64 {
    let commands = Commands { inner: input };
    let directory_sizes = all_directory_sizes(commands);

    let total_used_size = directory_sizes.get("/").unwrap();
    let remaining_free_space = 70000000 - total_used_size;
    let minimum_deletion = 30000000 - remaining_free_space;

    *directory_sizes
        .values()
        .filter(|value| **value >= minimum_deletion)
        .min()
        .unwrap()
}

fn all_directory_sizes(commands: Commands) -> HashMap<String, u64> {
    let mut directory_stack = vec![];
    let mut file_sizes = HashMap::new();
    let mut directory_contents = HashMap::new();
    for command in commands {
        if let CommandLineIO::CD(destination) = &command {
            match destination.as_ref() {
                ".." => {
                    directory_stack.pop();
                }
                dir => {
                    directory_stack.push(dir.to_owned());
                }
            };
        }
        if let CommandLineIO::Directory(name) | CommandLineIO::File(_, name) = &command {
            let current_directory = directory_stack.join("/");
            let full_name = format!("{}/{}", current_directory, name);
            directory_contents
                .entry(current_directory)
                .or_insert_with(HashSet::new)
                .insert(full_name.clone());
            if let CommandLineIO::File(size, _) = &command {
                file_sizes.insert(full_name, *size);
            }
        }
        // CommandLineIO::LS doesn't need to be handled
    }
    let mut directory_sizes = HashMap::new();
    directory_size("/", &mut directory_sizes, &directory_contents, &file_sizes);
    directory_sizes
}

fn directory_size(
    directory: &str,
    directory_sizes: &mut HashMap<String, u64>,
    directory_contents: &HashMap<String, HashSet<String>>,
    file_sizes: &HashMap<String, u64>,
) -> u64 {
    let mut result = 0;
    for content in directory_contents.get(directory).unwrap() {
        if let Some(file_size) = file_sizes.get(content) {
            result += file_size;
            continue;
        }
        if let Some(directory_size) = directory_sizes.get(content) {
            result += directory_size;
            continue;
        }
        // is directory, unknown size so recurse
        result += directory_size(content, directory_sizes, directory_contents, file_sizes);
    }
    directory_sizes.insert(directory.to_owned(), result);
    result
}

struct Commands<'a> {
    inner: &'a str,
}

impl<'a> Iterator for Commands<'a> {
    type Item = CommandLineIO;

    fn next(&mut self) -> Option<Self::Item> {
        if self.inner.is_empty() {
            return None;
        }
        match next_command(self.inner) {
            Ok((remaining, command)) => {
                self.inner = remaining;
                Some(command)
            }
            Err(e) => panic!("Unexpected error {e}"),
        }
    }
}

fn next_command(input: &str) -> IResult<&str, CommandLineIO> {
    alt((cd, ls, directory, file))(input)
}

fn cd(input: &str) -> IResult<&str, CommandLineIO> {
    map(
        delimited(tag("$ cd "), alt((tag(".."), tag("/"), alpha1)), newline),
        |s: &str| CommandLineIO::CD(s.to_string()),
    )(input)
}

fn ls(input: &str) -> IResult<&str, CommandLineIO> {
    map(pair(tag("$ ls"), newline), |_| CommandLineIO::LS)(input)
}

fn directory(input: &str) -> IResult<&str, CommandLineIO> {
    map(delimited(tag("dir "), alpha1, newline), |s: &str| {
        CommandLineIO::Directory(s.to_string())
    })(input)
}

fn file(input: &str) -> IResult<&str, CommandLineIO> {
    map(
        terminated(
            separated_pair(nom::character::complete::u64, multispace1, take_until("\n")),
            newline,
        ),
        |(size, name): (u64, &str)| CommandLineIO::File(size, name.to_string()),
    )(input)
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    static TEST_INPUT: &str = include_str!("../inputs/day07-test.txt");
    static FULL_INPUT: &str = include_str!("../inputs/day07.txt");

    #[test]
    fn part_1_test() {
        let output = super::part_1(TEST_INPUT);
        assert_eq!(output, 95437);
    }

    #[test]
    fn part_1() {
        let start = Instant::now();
        let output = super::part_1(FULL_INPUT);
        assert_eq!(output, 1723892);
        println!("Day 07 part 1 completed in {:?}", start.elapsed());
    }

    #[test]
    fn part_2_test() {
        let output = super::part_2(TEST_INPUT);
        assert_eq!(output, 24933642);
    }

    #[test]
    fn part_2() {
        let start = Instant::now();
        let output = super::part_2(FULL_INPUT);
        assert_eq!(output, 8474158);
        println!("Day 07 part 2 completed in {:?}", start.elapsed());
    }
}
