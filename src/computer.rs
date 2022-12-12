use nom::{
    branch::alt, bytes::complete::tag, character::complete::newline, combinator::map,
    sequence::delimited, IResult,
};

pub struct Computer {
    instructions: Vec<Instruction>,
    state: State,
}

#[derive(Copy, Clone)]
pub struct State {
    pub cycle_counter: i32,
    cycle: Cycle,
    program_counter: usize,
    pub register_x: i32,
    pub pixel: char,
}

#[derive(Copy, Clone)]
enum Cycle {
    First,
    Second,
}

impl Computer {
    pub fn new(input: &str) -> Computer {
        let instructions = InstructionParser { input }.collect();
        let cycle_counter = 0;
        let cycle = Cycle::First;
        let program_counter = 0;
        let register_x = 1;
        let pixel = '.';
        let state = State {
            cycle_counter,
            cycle,
            program_counter,
            register_x,
            pixel,
        };
        Computer {
            instructions,
            state,
        }
    }

    pub fn next_cycle(&mut self) -> Option<State> {
        self.state.cycle_counter += 1;
        self.state.pixel = self.get_pixel();
        let current_instruction = match self.instructions.get(self.state.program_counter).copied() {
            Some(instruction) => instruction,
            None => return None,
        };
        match self.state.cycle {
            Cycle::First => {
                match current_instruction {
                    Instruction::Noop => {
                        self.state.program_counter += 1;
                    }
                    Instruction::Addx(_) => {
                        self.state.cycle = Cycle::Second;
                    }
                };
                Some(self.state)
            }
            Cycle::Second => {
                self.state.cycle = Cycle::First;
                self.state.program_counter += 1;
                let stored_state = self.state;
                self.execute(&current_instruction);
                Some(stored_state)
            }
        }
    }

    fn execute(&mut self, current_instruction: &Instruction) {
        match current_instruction {
            Instruction::Noop => {}
            Instruction::Addx(arg) => {
                self.state.register_x += arg;
            }
        };
    }

    pub fn get_pixel(&self) -> char {
        let pixel = (self.state.cycle_counter - 1) % 40;
        if pixel == self.state.register_x - 1
            || pixel == self.state.register_x
            || pixel == self.state.register_x + 1
        {
            return '#';
        }
        '.'
    }
}
struct InstructionParser<'a> {
    input: &'a str,
}

impl<'a> Iterator for InstructionParser<'a> {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        if self.input.is_empty() {
            return None;
        }
        match next_instruction(self.input) {
            Ok((remaining, instruction)) => {
                self.input = remaining;
                Some(instruction)
            }
            Err(e) => panic!("Unexpected error {e}"),
        }
    }
}

#[derive(Copy, Clone)]
enum Instruction {
    Noop,
    Addx(i32),
}

fn next_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((noop, addx))(input)
}

fn noop(input: &str) -> IResult<&str, Instruction> {
    map(tag("noop\n"), |_: &str| Instruction::Noop)(input)
}

fn addx(input: &str) -> IResult<&str, Instruction> {
    map(
        delimited(tag("addx "), nom::character::complete::i32, newline),
        Instruction::Addx,
    )(input)
}
