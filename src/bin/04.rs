use advent_of_code::helpers::parse_input;

fn map_input_to_section_pairs(input: Vec<&str>) -> Vec<Vec<Vec<i32>>> {
    input
        .iter()
        .map(|lines| {
            lines
                .split(',')
                .map(|pairs| {
                    pairs
                        .split('-')
                        .map(|item| item.parse::<i32>().expect("Section parse to u32 failed!"))
                        .collect()
                })
                .collect()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let parsed_input = parse_input(input);
    let section_pairs = map_input_to_section_pairs(parsed_input);

    let result = section_pairs
        .iter()
        .map(|pairs| {
            let first_item_first_pair = pairs[0][0];
            let first_item_second_pair = pairs[1][0];

            let last_item_first_pair = pairs[0][1];
            let last_item_second_pair = pairs[1][1];

            let is_first_pair_container = {
                let first_diff = first_item_first_pair - first_item_second_pair;
                let last_diff = last_item_first_pair - last_item_second_pair;

                if first_diff.is_positive() || last_diff.is_negative() {
                    false
                } else {
                    true
                }
            };

            let is_second_pair_container = {
                let first_diff = first_item_second_pair - first_item_first_pair;
                let last_diff = last_item_second_pair - last_item_first_pair;

                if first_diff.is_positive() || last_diff.is_negative() {
                    false
                } else {
                    true
                }
            };

            is_first_pair_container || is_second_pair_container
        })
        .filter(|&is_contained| is_contained)
        .collect::<Vec<bool>>()
        .len();

    Some(result)
}

// NOTE: Look for any overlap in assignments!
pub fn part_two(input: &str) -> Option<u32> {
    let parsed_input = parse_input(input);
    let section_pairs = map_input_to_section_pairs(parsed_input);

    let result = section_pairs
        .iter()
        .filter(|pair| {
            let first_start = pair[0][0];
            let first_end = pair[0][1];
            let mut first_range = first_start..=first_end;

            let second_start = pair[1][0];
            let second_end = pair[1][1];
            let second_range = second_start..=second_end;

            first_range.any(|value| second_range.contains(&value))
        })
        .count() as u32;

    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
