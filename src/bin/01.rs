fn parse_input_into_snacks(input: &str) -> Vec<Vec<u32>> {
    input.split("\n\n")
        .map(|elf_snacks| elf_snacks
            .split('\n')
            .filter(|snack| !snack.is_empty())
            .map(|snack| snack.parse::<u32>().expect("Snack must be a number of calories!"))
            .collect()
        )
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let all_snacks = parse_input_into_snacks(input);
    let max_calories_snacks: u32 = all_snacks
        .iter()
        .map(|snacks| snacks.iter().fold(0, |acc, x| acc + x))
        .max()
        .expect("There should always be some snacks!");

    Some(max_calories_snacks)
}

pub fn part_two(input: &str) -> Option<u32> {
    let all_snacks = parse_input_into_snacks(input);
    let mut all_sorted_snacks: Vec<u32> = all_snacks
        .iter()
        .map(|snacks| snacks.iter().fold(0, |acc, x| acc + x))
        .collect();

    all_sorted_snacks.sort();

    let max_calories_snacks: u32 = all_sorted_snacks
        .iter()
        .rev()
        .take(3)
        .fold(0, |acc, x| acc + x);

    Some(max_calories_snacks)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
