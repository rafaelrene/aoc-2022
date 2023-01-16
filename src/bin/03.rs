#[derive(Debug)]
struct CharacterValue {
    character: char,
    value: u32,
}

impl CharacterValue {
    fn new(character: char) -> Self {
        let value = match character.is_uppercase() {
            true => (character as u32) - 38,
            false => (character as u32) - 96
        };

        Self {character, value}
    }

    fn get_value(self) -> u32 {
        self.value
    }
}

fn parse_input(input: &str) -> Vec<&str> {
    input
        .split("\n")
        .filter(|round| !round.is_empty())
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let parsed_input: u32 = parse_input(input)
        .iter()
        .map(|row| {
            let split_index = row.len() / 2;
            let (first_compartment_items, second_compartment_items) = row.split_at(split_index);

            return first_compartment_items
                .chars()
                .find(|first_group_letter| {
                    match second_compartment_items.chars().find(|second_group_letter| first_group_letter == second_group_letter) {
                        Some(..) => true,
                        _ => false
                    }
                })
                .expect("Char must exist here!");
        })
        .map(|letter| CharacterValue::new(letter).get_value())
        .sum();

    Some(parsed_input)
}

pub fn part_two(input: &str) -> Option<u32> {
    let parsed_input: Vec<&str> = parse_input(input);
    let elf_group_badges_value: u32 = parsed_input
        .chunks(3)
        .map(|elf_group| {
            let first_elf = elf_group[0];
            let second_elf = elf_group[1];
            let third_elf = elf_group[2];

            let badge: char = first_elf
                .chars()
                .find(|first_elf_item| {
                    let second_elf_matching_item = second_elf
                        .chars()
                        .find(|second_elf_item| first_elf_item == second_elf_item);

                    let third_elf_matching_item = third_elf
                        .chars()
                        .find(|third_elf_item| first_elf_item == third_elf_item);

                    match (second_elf_matching_item, third_elf_matching_item) {
                        (Some(..), Some(..)) => true,
                        (_, _) => false,
                    }
                })
                .expect("Badge must exist!");

            CharacterValue::new(badge).get_value()
        })
        .sum();

    Some(elf_group_badges_value)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
