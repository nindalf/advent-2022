use std::collections::{HashMap, HashSet};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{alpha1, multispace1, newline},
    combinator::map,
    multi::fold_many0,
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult,
};

#[derive(Debug)]
enum CliCommand {
    CD(String),
    LS(Vec<DirContent>),
}

#[derive(Debug)]
enum DirContent {
    Directory(String),
    File(String, u64),
}

/*
- Just wanted to write down my thoughts about solving this.
- Initial implementation stored state in multiple HashMaps - directory contents, file sizes, directory sizes
- Final calculations done on the directory sizes HashMap
- This didn't feel clean to me because surely this should just be a tree.
- Where it gets complicated is that it makes the final calculations harder.
- Those are quick and easy if we have a map of directory name to size.
- I can still do it. Generate a tree, and then a separate map for directory sizes.
- Meanwhile, despite spending a lot of time on the parser, I'm not happy with.
- Ideally the contents of LS should be parsed as a result of the LS command. (This is now complete)
- It's possible that a future day's challenge involves add support for another CLI command. In that unlikely event, I'm prepared!
 */

#[inline]
pub fn part_1(input: &str) -> u64 {
    let commands = Commands { inner: input };
    let directory_sizes = all_directory_sizes(commands);

    directory_sizes
        .values()
        .filter(|value| **value <= 100000)
        .sum()
}

#[inline]
pub fn part_2(input: &str) -> u64 {
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
        match &command {
            CliCommand::CD(destination) => match destination.as_ref() {
                ".." => {
                    directory_stack.pop();
                }
                dir => {
                    directory_stack.push(dir.to_owned());
                }
            },
            CliCommand::LS(dir_content) => {
                for content in dir_content {
                    match content {
                        DirContent::Directory(name) => {
                            let current_directory = directory_stack.join("/");
                            let full_name = format!("{}/{}", current_directory, name);
                            directory_contents
                                .entry(current_directory)
                                .or_insert_with(HashSet::new)
                                .insert(full_name.clone());
                        }
                        DirContent::File(name, size) => {
                            let current_directory = directory_stack.join("/");
                            let full_name = format!("{}/{}", current_directory, name);
                            directory_contents
                                .entry(current_directory)
                                .or_insert_with(HashSet::new)
                                .insert(full_name.clone());

                            file_sizes.insert(full_name, *size);
                        }
                    }
                }
            }
        };
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
    type Item = CliCommand;

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

fn next_command(input: &str) -> IResult<&str, CliCommand> {
    alt((cd, ls))(input)
}

fn cd(input: &str) -> IResult<&str, CliCommand> {
    map(
        delimited(tag("$ cd "), alt((tag(".."), tag("/"), alpha1)), newline),
        |s: &str| CliCommand::CD(s.to_string()),
    )(input)
}

fn ls(input: &str) -> IResult<&str, CliCommand> {
    map(
        preceded(
            tag("$ ls\n"),
            fold_many0(alt((directory, file)), Vec::new, |mut acc: Vec<_>, item| {
                acc.push(item);
                acc
            }),
        ),
        CliCommand::LS,
    )(input)
}

fn directory(input: &str) -> IResult<&str, DirContent> {
    map(delimited(tag("dir "), alpha1, newline), |s: &str| {
        DirContent::Directory(s.to_string())
    })(input)
}

fn file(input: &str) -> IResult<&str, DirContent> {
    map(
        terminated(
            separated_pair(nom::character::complete::u64, multispace1, take_until("\n")),
            newline,
        ),
        |(size, name): (u64, &str)| DirContent::File(name.to_string(), size),
    )(input)
}

#[cfg(test)]
mod tests {
    static TEST_INPUT: &str = include_str!("test-input.txt");
    static FULL_INPUT: &str = include_str!("input.txt");

    #[test]
    fn part_1_test() {
        let output = super::part_1(TEST_INPUT);
        assert_eq!(output, 95437);
    }

    #[test]
    fn part_1() {
        let output = super::part_1(FULL_INPUT);
        assert_eq!(output, 1723892);
    }

    #[test]
    fn part_2_test() {
        let output = super::part_2(TEST_INPUT);
        assert_eq!(output, 24933642);
    }

    #[test]
    pub fn part_2() {
        let output = super::part_2(FULL_INPUT);
        assert_eq!(output, 8474158);
    }
}
