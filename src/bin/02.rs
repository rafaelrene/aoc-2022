#[derive(Debug, Copy, Clone)]
enum RoundPoints {
    LOSS = 0,
    DRAW = 3,
    WIN = 6
}

#[derive(Debug, Copy, Clone)]
enum Shape {
    ROCK = 1,
    PAPER = 2,
    SCISSORS = 3
}

#[derive(Debug, Copy, Clone)]
struct Round {
    result: RoundPoints,
    my_shape: Shape,
}

fn assign_shape_by_char(choice: char) -> Shape {
    match choice {
        'A' | 'X' => Shape::ROCK,
        'B' | 'Y' => Shape::PAPER,
        'C' | 'Z' => Shape::SCISSORS,
        _ => panic!("Character mismatch during shape assignment!"),
    }
}

fn assign_round_points_by_char(choice: char) -> RoundPoints {
    match choice {
        'X' => RoundPoints::LOSS,
        'Y' => RoundPoints::DRAW,
        'Z' => RoundPoints::WIN,
        _ => panic!("Character mismatch during round points assignment!"),
    }
}

fn assign_shape_by_round_points(my_round_points: RoundPoints, player_one: Shape) -> Shape {
    match my_round_points {
        RoundPoints::WIN => player_one.get_winning_choice(),
        RoundPoints::DRAW => player_one.get_drawing_choice(),
        RoundPoints::LOSS => player_one.get_losing_choice(),
    }
}

fn play_round(player_one_shape: &Shape, player_two_shape: &Shape) -> RoundPoints {
    match (player_one_shape, player_two_shape) {
        (Shape::ROCK, Shape::PAPER) => RoundPoints::WIN,
        (Shape::PAPER, Shape::SCISSORS) => RoundPoints::WIN,
        (Shape::SCISSORS, Shape::ROCK) => RoundPoints::WIN,

        (Shape::ROCK, Shape::ROCK) => RoundPoints::DRAW,
        (Shape::PAPER, Shape::PAPER) => RoundPoints::DRAW,
        (Shape::SCISSORS, Shape::SCISSORS) => RoundPoints::DRAW,

        _ => RoundPoints::LOSS
    }
}

impl Shape {
    fn get_winning_choice(self) -> Self {
        match self {
            Shape::ROCK => Shape::PAPER,
            Shape::PAPER => Shape::SCISSORS,
            Shape::SCISSORS => Shape::ROCK,
        }
    }

    fn get_drawing_choice(self) -> Self {
        match self {
            Shape::ROCK => Shape::ROCK,
            Shape::PAPER => Shape::PAPER,
            Shape::SCISSORS => Shape::SCISSORS,
        }
    }

    fn get_losing_choice(self) -> Self {
        match self {
            Shape::ROCK => Shape::SCISSORS,
            Shape::PAPER => Shape::ROCK,
            Shape::SCISSORS => Shape::PAPER,
        }
    }
}

impl Round {
    fn new(player_one: Shape, player_two: Shape) -> Self {
        let result = play_round(&player_one, &player_two);
        Self {my_shape: player_two, result}
    }

    fn get_score(&self) -> u32 {
        self.result as u32 + self.my_shape as u32
    }
}


fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .split("\n")
        .filter(|round| !round.is_empty())
        .map(|round|
            round
                .split(' ')
                .map(|choice| choice.parse::<char>().expect("Could not parse as a character"))
                .collect()
        )
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let rounds = parse_input(input);
    let my_score: u32 = rounds
        .iter()
        .map(|round| {
            let player_one = assign_shape_by_char(round[0]);
            let player_two = assign_shape_by_char(round[1]);

            Round::new(player_one, player_two)
        })
        .fold(0, |acc, round| acc + round.get_score());

    Some(my_score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let rounds = parse_input(input);
    let my_score: u32 = rounds
        .iter()
        .map(|round| {
            let my_round_points = assign_round_points_by_char(round[1]);

            let player_one = assign_shape_by_char(round[0]);
            let player_two = assign_shape_by_round_points(my_round_points, player_one);

            Round::new(player_one, player_two)
        })
        .fold(0, |acc, round| acc + round.get_score());

    Some(my_score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
