use itertools::Itertools;
use nom::error::Error;
use nom::{
    bytes::complete::tag, combinator::map_res, multi::separated_list1, sequence::separated_pair,
    IResult,
};
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::fmt;
use std::ops;
use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    hash::Hash,
};

#[derive(Debug, Clone, PartialEq)]
enum Register {
    W,
    X,
    Y,
    Z,
}

#[derive(Clone, PartialEq)]
enum Operand {
    Literal(i64),
    Variable(Register),
}

#[derive(Clone, PartialEq)]
enum Instruction {
    Input(Register),
    Add(Register, Operand),
    Multiply(Register, Operand),
    Divide(Register, Operand),
    Modulus(Register, Operand),
    Equals(Register, Operand),
}

#[derive(Eq, PartialEq, Clone, Debug, Hash)]
struct ALUState {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
    input_so_far: Vec<i64>,
}

impl ALUState {
    fn new() -> Self {
        ALUState {
            w: 0,
            x: 0,
            y: 0,
            z: 0,
            input_so_far: vec![],
        }
    }

    fn from_z(z: i64) -> Self {
        ALUState {
            w: 0,
            x: 0,
            y: 0,
            z,
            input_so_far: vec![],
        }
    }

    fn execute(&self, instruction: &Instruction, input_stack: &mut Vec<i64>) -> Self {
        match instruction {
            Instruction::Input(register) => match input_stack.pop() {
                Some(value) => {
                    let mut next_state = self.set_register(register, value);
                    next_state.input_so_far.push(value);
                    next_state
                }
                None => {
                    panic!("attempted to read input with no input left")
                }
            },
            Instruction::Add(register, operand) => self.set_register(
                register,
                self.read_register(register) + self.read_operand(operand),
            ),
            Instruction::Multiply(register, operand) => self.set_register(
                register,
                self.read_register(register) * self.read_operand(operand),
            ),
            Instruction::Divide(register, operand) => self.set_register(
                register,
                self.read_register(register) / self.read_operand(operand),
            ),
            Instruction::Modulus(register, operand) => self.set_register(
                register,
                self.read_register(register) % self.read_operand(operand),
            ),
            Instruction::Equals(register, operand) => self.set_register(
                register,
                if self.read_register(register) == self.read_operand(operand) {
                    1
                } else {
                    0
                },
            ),
        }
    }

    fn set_register(&self, register: &Register, value: i64) -> Self {
        match register {
            Register::W => ALUState {
                w: value,
                x: self.x,
                y: self.y,
                z: self.z,
                input_so_far: self.input_so_far.clone(),
            },
            Register::X => ALUState {
                x: value,
                w: self.w,
                y: self.y,
                z: self.z,
                input_so_far: self.input_so_far.clone(),
            },
            Register::Y => ALUState {
                y: value,
                w: self.w,
                x: self.x,
                z: self.z,
                input_so_far: self.input_so_far.clone(),
            },
            Register::Z => ALUState {
                z: value,
                w: self.w,
                x: self.x,
                y: self.y,
                input_so_far: self.input_so_far.clone(),
            },
        }
    }

    fn read_register(&self, register: &Register) -> i64 {
        match register {
            Register::W => self.w,
            Register::X => self.x,
            Register::Y => self.y,
            Register::Z => self.z,
        }
    }

    fn read_operand(&self, operand: &Operand) -> i64 {
        match operand {
            Operand::Literal(value) => *value,
            Operand::Variable(register) => self.read_register(register),
        }
    }

    fn print(&self) {
        println!("W: {}, X: {}, Y: {}, Z: {}", self.w, self.x, self.y, self.z);
    }
}

impl Ord for ALUState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.z.cmp(&other.z)
    }
}

impl PartialOrd for ALUState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instruction::Input(register) => {
                write!(f, "read input into {:?}", register)
            }
            Instruction::Add(register, operand) => {
                write!(f, "{:?} <- {:?} + {:?}", register, register, operand)
            }
            Instruction::Multiply(register, operand) => {
                write!(f, "{:?} <- {:?} * {:?}", register, register, operand)
            }
            Instruction::Divide(register, operand) => {
                write!(f, "{:?} <- {:?} / {:?}", register, register, operand)
            }
            Instruction::Modulus(register, operand) => {
                write!(f, "{:?} <- {:?} % {:?}", register, register, operand)
            }
            Instruction::Equals(register, operand) => {
                write!(
                    f,
                    "{:?} <- {:?} = {:?} ? 1 : 0",
                    register, register, operand
                )
            }
        }
    }
}

impl fmt::Debug for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operand::Variable(register) => {
                write!(f, "{:?}", register)
            }
            Operand::Literal(literal) => {
                write!(f, "{}", literal)
            }
        }
    }
}

pub fn part1() -> u64 {
    let input = include_str!("../../data/2021/24.txt");

    match input_parser(input) {
        Ok((remaining_input, nomad_program)) if remaining_input.is_empty() => {
            println!("parsed entire input");

            let grouped_by_input = split_by_input(&nomad_program);

            let mut pushes: Vec<usize> = vec![];
            let mut pairs: Vec<(usize, usize)> = vec![];

            for (index, group) in grouped_by_input.iter().enumerate() {
                if group.get(4).unwrap() == &Instruction::Divide(Register::Z, Operand::Literal(1)) {
                    //push
                    pushes.push(index);
                } else {
                    //pop
                    let paired_push = pushes.pop().unwrap();
                    pairs.push((paired_push, index));
                }
            }

            let mut digits: Vec<i64> = vec![0; 14];

            for (push, pop) in pairs {
                let mut found_pair = false;
                for first_input in (1..10).rev() {
                    for second_input in (1..10).rev() {
                        let resulting_state = run_program_segment(
                            &grouped_by_input[push],
                            first_input,
                            ALUState::new(),
                        );
                        let resulting_state = run_program_segment(
                            &grouped_by_input[pop],
                            second_input,
                            resulting_state,
                        );

                        if resulting_state.z == 0 {
                            digits[push] = first_input;
                            digits[pop] = second_input;
                            found_pair = true;
                            break;
                        }
                    }

                    if found_pair {
                        break;
                    }
                }
            }

            println!("found the digits: {:?}", digits);

            input_to_int(&digits)
        }
        Ok((remaining, _)) => {
            println!("remaining unparsed \"{}\"", remaining);
            0
        }
        Err(e) => {
            println!("error parsing \"{}\"", e);
            0
        }
    }
}

pub fn part2() -> u64 {
    let input = include_str!("../../data/2021/24.txt");

    match input_parser(input) {
        Ok((remaining_input, nomad_program)) if remaining_input.is_empty() => {
            println!("parsed entire input");

            println!("parsed entire input");

            let grouped_by_input = split_by_input(&nomad_program);

            let mut pushes: Vec<usize> = vec![];
            let mut pairs: Vec<(usize, usize)> = vec![];

            for (index, group) in grouped_by_input.iter().enumerate() {
                if group.get(4).unwrap() == &Instruction::Divide(Register::Z, Operand::Literal(1)) {
                    //push
                    pushes.push(index);
                } else {
                    //pop
                    let paired_push = pushes.pop().unwrap();
                    pairs.push((paired_push, index));
                }
            }

            let mut digits: Vec<i64> = vec![0; 14];

            for (push, pop) in pairs {
                let mut found_pair = false;
                for first_input in 1..10 {
                    for second_input in 1..10 {
                        let resulting_state = run_program_segment(
                            &grouped_by_input[push],
                            first_input,
                            ALUState::new(),
                        );
                        let resulting_state = run_program_segment(
                            &grouped_by_input[pop],
                            second_input,
                            resulting_state,
                        );

                        if resulting_state.z == 0 {
                            digits[push] = first_input;
                            digits[pop] = second_input;
                            found_pair = true;
                            break;
                        }
                    }

                    if found_pair {
                        break;
                    }
                }
            }

            println!("found the digits: {:?}", digits);

            input_to_int(&digits)
        }
        Ok((remaining, _)) => {
            println!("remaining unparsed \"{}\"", remaining);
            0
        }
        Err(e) => {
            println!("error parsing \"{}\"", e);
            0
        }
    }
}

fn input_to_int(input: &Vec<i64>) -> u64 {
    let mut result: u64 = 0;
    for digit in input {
        result *= 10;
        result += *digit as u64;
    }

    result
}

fn next_input(input: &Vec<i64>) -> Option<Vec<i64>> {
    let mut generated_input = input.to_owned();

    generated_input.reverse();

    let mut rolled = false;
    let mut done = false;
    for value in generated_input.iter_mut() {
        if rolled {
            if *value > 1 {
                *value -= 1;
                done = true;
                break;
            } else {
                *value = 9;
            }
        } else if *value > 1 {
            *value -= 1;
            done = true;
            break;
        } else {
            rolled = true;
            *value = 9;
        }
    }

    if done {
        generated_input.reverse();
        Some(generated_input)
    } else {
        None
    }
}

fn advance_input_when_z_is_zero(program: &Vec<Instruction>) {
    //try piecing together the number one input at a time
    //start with 9 and go down to 1 until the result is 0
    //on each input, save the previous input to the input stack
    //if z > 0, then rewind to the last input step
    //if there are multiple "solutions" to a part, then we'll have to keep a set of possibilities per index

    let mut guessed_input = vec![];
    let mut accepted_program_len: usize = 0;

    let mut state;

    let mut state_save_point = ALUState::new();
    let mut this_guess: i64 = 10;

    while guessed_input.len() < 14 {
        state = state_save_point.clone();
        this_guess -= 1;

        for (instruction_read_so_far, instruction) in
            program.iter().skip(accepted_program_len).enumerate()
        {
            if let Instruction::Input(_) = instruction {
                if instruction_read_so_far > 0 {
                    //then this isn't our first input
                    if state.z == 0 {
                        guessed_input.push(this_guess);
                        state_save_point = state.clone();
                        this_guess = 10;
                        accepted_program_len += instruction_read_so_far - 1;
                    }

                    break;
                }
            }

            state = state.execute(instruction, &mut vec![this_guess]);
        }

        if this_guess == 1 {
            println!("failed with input at 1. would need to rewind input.");
            return;
        }
    }

    println!("succeeded with input {:?}", guessed_input);
}

fn split_by_input(program: &Vec<Instruction>) -> Vec<Vec<Instruction>> {
    let mut grouped_by_instruction: Vec<Vec<Instruction>> = vec![];
    let mut accepted_program_len: usize = 0;

    while grouped_by_instruction.len() < 14 {
        let mut this_group: Vec<Instruction> = vec![];

        for (instruction_read_so_far, instruction) in
            program.iter().skip(accepted_program_len).enumerate()
        {
            if let Instruction::Input(_) = instruction {
                if instruction_read_so_far > 0 {
                    //then this isn't our first input
                    accepted_program_len += instruction_read_so_far;
                    break;
                }
            }

            this_group.push(instruction.to_owned());
        }

        grouped_by_instruction.push(this_group);
    }

    grouped_by_instruction
}

fn run_program(program: &Vec<Instruction>, input: &Vec<i64>) -> Option<i64> {
    let mut state = ALUState::new();

    let mut input_stack = input.clone();
    input_stack.reverse();

    for (instruction_counter, instruction) in program.iter().enumerate() {
        state = state.execute(instruction, &mut input_stack);

        if state.z != 0 {
            return None;
        }
    }

    Some(state.z)
}

fn run_program_segment(
    program: &Vec<Instruction>,
    input: i64,
    initial_state: ALUState,
) -> ALUState {
    let mut state = initial_state;

    let mut input_stack = vec![input];

    for instruction in program {
        state = state.execute(instruction, &mut input_stack);
    }

    state
}

fn debug_program(program: &Vec<Instruction>, input: &Vec<i64>) -> Option<i64> {
    let mut state = ALUState::new();

    let mut input_stack = input.clone();
    input_stack.reverse();

    state.print();

    for instruction in program {
        println!("{:?}", instruction);
        state = state.execute(instruction, &mut input_stack);
        state.print();
    }

    Some(state.z)
}

fn parse_number(input: &str) -> IResult<&str, i64> {
    let (input, maybe_negative) = nom::combinator::opt(tag("-"))(input)?;
    let (input, val) =
        map_res(nom::character::complete::digit1, |s: &str| s.parse::<i64>())(input)?;

    Ok((input, if maybe_negative.is_some() { -val } else { val }))
}

fn register_parser(input: &str) -> IResult<&str, Register> {
    let (input, register) = nom::character::complete::alpha1(input)?;

    match register {
        "w" => Ok((input, Register::W)),
        "x" => Ok((input, Register::X)),
        "y" => Ok((input, Register::Y)),
        "z" => Ok((input, Register::Z)),
        _ => panic!("Invalid register"),
    }
}

fn register_operand_parser(input: &str) -> IResult<&str, Operand> {
    let (input, register) = register_parser(input)?;

    Ok((input, (Operand::Variable(register))))
}

fn literal_parser(input: &str) -> IResult<&str, Operand> {
    let (input, value) = parse_number(input)?;

    Ok((input, Operand::Literal(value)))
}

fn operand_parser(input: &str) -> IResult<&str, Operand> {
    nom::branch::alt((literal_parser, register_operand_parser))(input)
}

fn instruction_parser(input: &str) -> IResult<&str, Instruction> {
    let (input, operator) = nom::character::complete::alpha1(input)?;
    let (input, _) = tag(" ")(input)?;

    match operator {
        "inp" => {
            let (input, register) = register_parser(input)?;
            Ok((input, Instruction::Input(register)))
        }
        "add" => {
            let (input, register) = register_parser(input)?;
            let (input, _) = tag(" ")(input)?;
            let (input, operand) = operand_parser(input)?;
            Ok((input, Instruction::Add(register, operand)))
        }
        "mul" => {
            let (input, register) = register_parser(input)?;
            let (input, _) = tag(" ")(input)?;
            let (input, operand) = operand_parser(input)?;
            Ok((input, Instruction::Multiply(register, operand)))
        }
        "div" => {
            let (input, register) = register_parser(input)?;
            let (input, _) = tag(" ")(input)?;
            let (input, operand) = operand_parser(input)?;
            Ok((input, Instruction::Divide(register, operand)))
        }
        "mod" => {
            let (input, register) = register_parser(input)?;
            let (input, _) = tag(" ")(input)?;
            let (input, operand) = operand_parser(input)?;
            Ok((input, Instruction::Modulus(register, operand)))
        }
        "eql" => {
            let (input, register) = register_parser(input)?;
            let (input, _) = tag(" ")(input)?;
            let (input, operand) = operand_parser(input)?;
            Ok((input, Instruction::Equals(register, operand)))
        }
        _ => panic!("invalid operator"),
    }
}

fn input_parser(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, numbers) =
        separated_list1(nom::character::complete::newline, instruction_parser)(input)?;

    let (input, _) = nom::character::complete::newline(input)?;

    Ok((input, numbers))
}

#[cfg(test)]
mod day24_tests {
    use crate::day24::ALUState;
    use crate::day24::Instruction;
    use crate::day24::Operand;
    use crate::day24::Register;

    #[test]
    fn test_divide() {
        let mut input = vec![-5];
        let negative_divide = ALUState::new()
            .execute(&Instruction::Input(Register::W), &mut input)
            .execute(
                &Instruction::Divide(Register::W, Operand::Literal(2)),
                &mut input,
            );
        assert_eq!(negative_divide.w, -2);
    }
}
