use std::{
    collections::{HashMap, HashSet},
    fs,
};

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
        let (numbers, _) = self.get_values();
        println!("Part 1 - Result: {:?}", numbers.iter().sum::<usize>());
    }

    fn part_2(&self) {
        let (_, adjacent_parts) = self.get_values();
        let result: usize = adjacent_parts
            .values()
            .filter(|s| s.len() == 2)
            .map(|s| s.iter().fold(1, |acc, v| v * acc))
            .sum();
        println!("Part 2 - Result: {:?}", result);
    }

    fn get_values(&self) -> (Vec<usize>, HashMap<String, HashSet<usize>>) {
        let contents: Vec<char> = fs::read_to_string("inputs/day_3.txt")
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

        // last loop aggregation needs to be appended to the matrix
        // as the last loop around does reset the accumulator.
        matrix.push(temp_arr);

        let mut numbers: Vec<usize> = Vec::new();
        let mut current_values: Vec<CharAt> = Vec::new();

        let mut adjacent_parts: HashMap<String, HashSet<usize>> = HashMap::new();

        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                let value = CharAt::new(i, j, matrix[i][j]);
                if value.value.is_ascii_digit() {
                    current_values.push(value);
                } else {
                    let mut is_adjacent = false;

                    let mut temp_star: Option<CharAt> = None;

                    for (index, v) in current_values.iter().enumerate() {
                        let has_left = v.col > 0 && index == 0;
                        let has_above = v.row > 0;
                        let has_right =
                            v.col < matrix[i].len() - 1 && index == current_values.len() - 1;
                        let has_bottom = v.row < matrix.len() - 1;

                        if has_left {
                            let left_value = matrix[v.row][v.col - 1];
                            if is_simbol(left_value) {
                                is_adjacent = true;
                                if is_star(left_value) {
                                    temp_star = Some(CharAt::new(v.row, v.col - 1, left_value));
                                }
                            }
                        }
                        if has_above {
                            let above_value = matrix[v.row - 1][v.col];
                            if is_simbol(above_value) {
                                is_adjacent = true;
                                if is_star(above_value) {
                                    temp_star = Some(CharAt::new(v.row - 1, v.col, above_value));
                                }
                            }
                        }

                        // check left for diagonals
                        if has_left && has_above {
                            let top_left = matrix[v.row - 1][v.col - 1];
                            if is_simbol(top_left) {
                                is_adjacent = true;
                                if is_star(top_left) {
                                    temp_star = Some(CharAt::new(v.row - 1, v.col - 1, top_left));
                                }
                            }
                        }

                        if has_left && has_bottom {
                            let bottom_left = matrix[v.row + 1][v.col - 1];
                            if is_simbol(bottom_left) {
                                is_adjacent = true;
                                if is_star(bottom_left) {
                                    temp_star =
                                        Some(CharAt::new(v.row + 1, v.col - 1, bottom_left));
                                }
                            }
                        }

                        if has_right {
                            let right_value = matrix[v.row][v.col + 1];
                            if is_simbol(right_value) {
                                is_adjacent = true;
                                if is_star(right_value) {
                                    temp_star = Some(CharAt::new(v.row, v.col + 1, right_value));
                                }
                            }
                        }

                        if has_bottom {
                            let bottom_value = matrix[v.row + 1][v.col];
                            if is_simbol(bottom_value) {
                                is_adjacent = true;
                                if is_star(bottom_value) {
                                    temp_star = Some(CharAt::new(v.row + 1, v.col, bottom_value));
                                }
                            }
                        }

                        // check right for diagonals
                        if has_right && has_above {
                            let top_right = matrix[v.row - 1][v.col + 1];
                            if is_simbol(top_right) {
                                is_adjacent = true;
                                if is_star(top_right) {
                                    temp_star = Some(CharAt::new(v.row - 1, v.col + 1, top_right));
                                }
                            }
                        }

                        if has_right && has_bottom {
                            let bottom_right = matrix[v.row + 1][v.col + 1];
                            if is_simbol(bottom_right) {
                                is_adjacent = true;
                                if is_star(bottom_right) {
                                    temp_star =
                                        Some(CharAt::new(v.row + 1, v.col + 1, bottom_right));
                                }
                            }
                        }
                    }

                    if is_adjacent {
                        let number = current_values
                            .iter()
                            .fold("".to_string(), |acc, c| format!("{}{}", acc, c.value))
                            .parse::<usize>()
                            .unwrap();
                        numbers.push(number);

                        if let Some(star) = temp_star {
                            let key = format!("{}-{}", star.row, star.col);
                            match adjacent_parts.get_mut(&key) {
                                Some(set) => {
                                    set.insert(number);
                                }
                                None => {
                                    let new_set = HashSet::from([number]);
                                    adjacent_parts.insert(key, new_set);
                                }
                            }
                        }
                    }

                    // reset accumulator
                    current_values = Vec::new();
                }
            }
        }

        (numbers, adjacent_parts)
    }
}

fn is_simbol(c: char) -> bool {
    !c.is_ascii_digit() && c != '.'
}

fn is_star(c: char) -> bool {
    c == '*'
}

#[derive(Debug)]
struct CharAt {
    row: usize,
    col: usize,
    value: char,
}

impl CharAt {
    fn new(row: usize, col: usize, value: char) -> Self {
        Self { row, col, value }
    }
}
