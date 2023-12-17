use std::{fs, time::Instant};

pub struct Day4 {}

impl Day4 {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self) {
        let start = Instant::now();
        self.part_1();
        println!("Part 1 time: {:?}", start.elapsed());
        let start = Instant::now();
        self.part_2();
        println!("Part 2 time: {:?}", start.elapsed());
    }

    // See: https://adventofcode.com/2023/day/3
    fn part_1(&self) {
        let cards = self.parse();
        let points: usize = cards.iter().map(|c| c.points()).sum();
        println!("Part 1 - Result: {:?}", points);
    }

    fn part_2(&self) {
        println!("Part 2 - Result: {:?}", "TODO");
    }

    fn parse(&self) -> Vec<Card> {
        let contents = fs::read_to_string("inputs/day_4.txt").unwrap();
        contents.lines().map(Card::new).collect()
    }
}

#[derive(Debug)]
struct Card {
    winning_numbers: Vec<usize>,
    given_numbers: Vec<usize>,
}

impl Card {
    fn new(line: &str) -> Self {
        let inputs: Vec<&str> = line.split('|').collect();

        let to_list = |input: &str| -> Vec<usize> {
            input
                .split(' ')
                .map(|v| v.trim())
                .filter(|v| !v.is_empty())
                .map(|v| v.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        };

        let winning_numbers = to_list(inputs[0].split(':').collect::<Vec<&str>>()[1]);
        let given_numbers = to_list(inputs[1]);

        Self {
            winning_numbers,
            given_numbers,
        }
    }
    fn points(&self) -> usize {
        let mut points: usize = 0;

        for winner in &self.winning_numbers {
            if self.given_numbers.contains(winner) {
                if points == 0 {
                    points = 1;
                } else {
                    points *= 2;
                }
            }
        }

        points
    }
}
