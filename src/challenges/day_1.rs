use std::fs;

pub struct Day1 {}

impl Day1 {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(self) {
        self.part_1()
    }

    // See: https://adventofcode.com/2023/day/1
    fn part_1(self) {
        let contents = fs::read_to_string("inputs/day_1.txt").unwrap();
        let mut numbers: Vec<usize> = Vec::with_capacity(contents.lines().count());

        for line in contents.lines() {
            let mut first_num: Option<char> = None;
            let mut last_num: Option<char> = None;
            let size = line.chars().count();
            for i in 0..size {
                let left_char = line.chars().nth(i).unwrap();
                let right_char = line.chars().nth((size - 1) - i).unwrap();

                if first_num.is_none() && left_char.is_ascii_digit() {
                    first_num = Some(left_char)
                }

                if last_num.is_none() && right_char.is_ascii_digit() {
                    last_num = Some(right_char)
                }

                if let (Some(first), Some(last)) = (first_num, last_num) {
                    let num: usize = format!("{}{}", first, last).parse().unwrap();
                    numbers.push(num);
                    break;
                }
            }
        }

        let result: usize = numbers.iter().sum();
        println!("Result: {}", result)
    }
}
