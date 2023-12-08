use std::fs;

pub struct Day3 {}

impl Day3 {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self) {
        self.part_1();
        self.part_2();
    }

    // See: https://adventofcode.com/2023/day/3
    fn part_1(&self) {
        let contents: Vec<char> = fs::read_to_string("inputs/day_3_part_1.txt")
            .unwrap()
            .chars()
            .collect();

        let mut matrix: Vec<Vec<char>> = Vec::new();
        let mut temp_arr: Vec<char> = Vec::new();

        let mut c = 0;
        while c < contents.len() {
            if contents[c] == '\n' {
                matrix.push(temp_arr);
                temp_arr = Vec::new();
            } else {
                temp_arr.push(contents[c]);
            }

            c += 1;
        }

        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                let value = matrix[i][j];
                print!("{value} ")
            }
        }
    }

    fn part_2(&self) {
        println!("Part 2 - Result: todo");
    }
}
