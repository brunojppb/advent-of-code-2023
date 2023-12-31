use std::{collections::HashMap, fs};

pub struct Day1 {}

impl Day1 {
    pub fn new() -> Self {
        Self {}
    }

    // See: https://adventofcode.com/2023/day/1
    pub fn part_1(&self) -> usize {
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

        numbers.iter().sum::<usize>()
    }

    fn part_2(&self) -> usize {
        let digits = HashMap::from([
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ]);

        let contents = fs::read_to_string("inputs/day_1.txt").unwrap();
        let mut numbers: Vec<usize> = Vec::new();

        for line in contents.lines() {
            let mut acc = String::new();
            let size = line.chars().count();
            for i in 0..size {
                let char = line.chars().nth(i).unwrap();
                if char.is_ascii_digit() {
                    acc = format!("{}{}", acc, char);
                    continue;
                }

                for (key, num) in &digits {
                    let range = i..(i + (key.len()));
                    if size - i >= key.len() {
                        let slice = &line[range];
                        if &slice == key {
                            acc = format!("{}{}", acc, num);
                            continue;
                        }
                    }
                }
            }
            let (first, last) = (acc.chars().nth(0), acc.chars().nth(acc.len() - 1));
            numbers.push(
                format!("{}{}", first.unwrap(), last.unwrap())
                    .parse()
                    .unwrap(),
            );
        }

        numbers.into_iter().sum::<usize>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let day_1 = Day1::new();
        let result = day_1.part_1();
        assert_eq!(result, 54081);
    }

    #[test]
    fn part_2() {
        let day_1 = Day1::new();
        let result = day_1.part_2();
        assert_eq!(result, 54649);
    }
}
