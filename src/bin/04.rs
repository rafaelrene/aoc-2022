use advent_of_code::helpers::parse_input;

fn map_input_to_section_pairs(input: Vec<&str>) -> Vec<Vec<Vec<i32>>> {
    input
        .iter()
        .map(|pairs| pairs
            .split(',')
            .map(|pair| pair
                .split('-')
                .map(|section| section.parse::<i32>().expect("Section parse to u32 failed!"))
                .collect::<Vec<i32>>()
            )
            .collect::<Vec<Vec<i32>>>()
        )
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

                if first_diff.is_positive() || last_diff.is_negative() {false} else {true}
            };

            let is_second_pair_container =  {
                let first_diff = first_item_second_pair - first_item_first_pair;
                let last_diff = last_item_second_pair - last_item_first_pair;

                if first_diff.is_positive() || last_diff.is_negative() {false} else {true}
            };

            is_first_pair_container || is_second_pair_container
        })
        .filter(|&is_contained| is_contained)
        .collect::<Vec<bool>>()
        .len();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let _parsed_input = parse_input(input);

    None
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
        assert_eq!(part_two(&input), None);
    }
}
