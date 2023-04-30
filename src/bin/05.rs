use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref CRATE_REGEX: Regex =
        Regex::new(r"(\[(?P<character>[A-Z])\]|\s{3})\s?").expect("CRATE_REGEX compiles");
    static ref INSTRUCTION_REGEX: Regex =
        Regex::new(r"move (?P<amount>\d+) from (?P<fromIndex>\d+) to (?P<toIndex>\d+)")
            .expect("INSTRUCTION_REGEX compiles");
}

#[derive(Debug)]
struct Instruction {
    amount: usize,
    from_index: usize,
    to_index: usize,
}

#[derive(PartialEq)]
enum CraneType {
    CrateMover9000,
    CrateMover9001,
}

fn split_input(input: &str) -> (&str, &str) {
    let result: Vec<&str> = input.split("\n\n").collect();

    return (result[0], result[1]);
}

fn get_stacks_from_drawing(drawing: &str) -> Vec<Vec<char>> {
    let heap: Vec<Vec<Option<char>>> = drawing
        .lines()
        .filter(|line| line.contains('['))
        .map(|line| {
            let mut crates: Vec<Option<char>> = vec![];

            for crate_match in CRATE_REGEX.captures_iter(line) {
                crates.push(crate_match.name("character").map(|m| {
                    return m
                        .as_str()
                        .chars()
                        .next()
                        .expect("single char in capture group");
                }))
            }

            return crates;
        })
        .collect();

    let stack_count = heap[0].len();
    let mut stacks = vec![vec![]; stack_count];

    heap.iter().rev().for_each(|stack| {
        for (index, item) in stack.iter().enumerate() {
            if let Some(item) = item {
                stacks[index].push(*item);
            }
        }
    });

    return stacks;
}

fn get_instructions_from_procedures(procedures: &str) -> Vec<Instruction> {
    return procedures
        .lines()
        .map(|line| match INSTRUCTION_REGEX.captures(line) {
            Some(captures) => Instruction {
                amount: captures
                    .name("amount")
                    .expect("Group 'amount' unmatched!")
                    .as_str()
                    .parse()
                    .expect("'amount' not parsed!"),
                from_index: captures
                    .name("fromIndex")
                    .expect("Group 'fromIndex' unmatched!")
                    .as_str()
                    .parse()
                    .expect("'fromIndex' not parsed!"),
                to_index: captures
                    .name("toIndex")
                    .expect("Group 'to_index' unmatched!")
                    .as_str()
                    .parse()
                    .expect("'to_index' not parsed!"),
            },
            None => panic!("Instruction not matching regex: {:?}", line),
        })
        .collect();
}

fn perform_instruction(
    instruction: &Instruction,
    stacks: &mut Vec<Vec<char>>,
    crane_type: &CraneType,
) {
    match crane_type {
        CraneType::CrateMover9000 => {
            for _ in 0..instruction.amount {
                let item = stacks[instruction.from_index - 1]
                    .pop()
                    .expect("take crate from stack");

                stacks[instruction.to_index - 1].push(item);
            }
        }
        CraneType::CrateMover9001 => {
            unimplemented!()
        }
    }
}

fn process(
    mut stacks: Vec<Vec<char>>,
    instructions: Vec<Instruction>,
    crane_type: CraneType,
) -> String {
    instructions
        .iter()
        .for_each(|instruction| perform_instruction(instruction, &mut stacks, &crane_type));

    return stacks
        .iter()
        .map(|s| s.last().expect("has last crate"))
        .collect::<String>();
}

pub fn part_one(input: &str) -> Option<String> {
    let (drawing, procedures) = split_input(input);

    let stacks = get_stacks_from_drawing(drawing);
    let instructions = get_instructions_from_procedures(procedures);

    return Some(process(stacks, instructions, CraneType::CrateMover9000));
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), None);
    }
}
