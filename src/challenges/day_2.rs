use std::fs;

pub struct Day2 {}

impl Day2 {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self) {
        self.part_1();
        self.part_2();
    }

    // See: https://adventofcode.com/2023/day/2
    fn part_1(&self) {
        let contents = fs::read_to_string("inputs/day_2_part_1.txt").unwrap();
        let games = contents.lines().map(Game::parse);
        let result: u32 = games.filter(|g| g.is_valid()).map(|g| g.id).sum();
        println!("Part 1 - Result: {}", result);
    }

    fn part_2(&self) {
        let contents = fs::read_to_string("inputs/day_2_part_2.txt").unwrap();
        let games = contents.lines().map(Game::parse);
        let result: u32 = games.map(|g| g.power()).sum();
        println!("Part 2 - Result: {}", result);
    }
}

const MAX_RED_CUBES: u32 = 12;
const MAX_GREEN_CUBES: u32 = 13;
const MAX_BLUE_CUBES: u32 = 14;

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}
#[derive(Debug)]
struct Round {
    rolls: Vec<Color>,
}

#[derive(Debug)]
enum Color {
    Red(u32),
    Blue(u32),
    Green(u32),
}

impl Game {
    fn parse(input: &str) -> Self {
        let first_part: Vec<&str> = input.split(':').collect();
        let (id_part, rounds_part) = (first_part[0], first_part[1]);
        let id_split: Vec<&str> = id_part.split(' ').collect();
        let id = id_split[1];

        let round_values: Vec<&str> = rounds_part.trim().split(';').collect();
        let mut rounds: Vec<Round> = Vec::with_capacity(round_values.len());
        for value in round_values {
            let round = Round::parse(value);
            rounds.push(round);
        }

        Self {
            id: id.parse().unwrap(),
            rounds,
        }
    }

    fn is_valid(&self) -> bool {
        self.rounds.iter().all(|r| r.is_valid())
    }

    fn power(&self) -> u32 {
        let mut green: u32 = 1;
        let mut blue: u32 = 1;
        let mut red: u32 = 1;

        for round in &self.rounds {
            for roll in &round.rolls {
                match *roll {
                    Color::Blue(v) => {
                        if blue < v {
                            blue = v
                        }
                    }
                    Color::Red(v) => {
                        if red < v {
                            red = v
                        }
                    }
                    Color::Green(v) => {
                        if green < v {
                            green = v
                        }
                    }
                }
            }
        }

        blue * red * green
    }
}

impl Round {
    fn parse(input: &str) -> Self {
        let round_values: Vec<&str> = input.trim().split(',').collect();
        let mut roll_values: Vec<Color> = Vec::with_capacity(round_values.len());
        for str_value in round_values {
            let values: Vec<&str> = str_value.trim().split(' ').collect();
            let (num_str, color_name) = (values[0], values[1]);
            let result = match color_name {
                "blue" => Color::Blue(num_str.parse().unwrap()),
                "green" => Color::Green(num_str.parse().unwrap()),
                "red" => Color::Red(num_str.parse().unwrap()),
                invalid_color => unreachable!("invalid color value '{}'", invalid_color),
            };
            roll_values.push(result);
        }

        Self { rolls: roll_values }
    }

    fn is_valid(&self) -> bool {
        self.rolls.iter().all(|roll| match *roll {
            Color::Blue(v) => v <= MAX_BLUE_CUBES,
            Color::Red(v) => v <= MAX_RED_CUBES,
            Color::Green(v) => v <= MAX_GREEN_CUBES,
        })
    }
}
