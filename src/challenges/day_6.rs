use std::fs;
use std::time::Instant;

pub struct Day6 {}

impl Day6 {
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

    // See: https://adventofcode.com/2023/day/5
    fn part_1(&self) -> usize {
        println!("TODO!");
        1
    }

    fn part_2(&self) -> usize {
        println!("TODO!");
        1
    }

    fn _parse(&self) {
        let _contents = fs::read_to_string("inputs/day_6.txt").unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore = "Not implemented yet"]
    #[test]
    fn part_1() {
        let day_6 = Day6::new();
        let result = day_6.part_1();
        assert_eq!(result, 1);
    }

    #[ignore = "Not implemented yet"]
    #[test]
    fn part_2() {
        let day_6 = Day6::new();
        let result = day_6.part_2();
        assert_eq!(result, 1);
    }
}
