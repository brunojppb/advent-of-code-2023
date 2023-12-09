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

        // last loop aggregation needs to be appended to the matrix
        // as the last loop around does reset the accumulator.
        matrix.push(temp_arr);

        let mut numbers: Vec<usize> = Vec::new();
        let mut current_values: Vec<CharAt> = Vec::new();

        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                let value = CharAt::new(i, j, matrix[i][j]);
                if value.value.is_ascii_digit() {
                    current_values.push(value);
                } else {
                    let mut is_adjacent = false;

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
                                break;
                            }
                        }
                        if has_above {
                            let above_value = matrix[v.row - 1][v.col];
                            if is_simbol(above_value) {
                                is_adjacent = true;
                                break;
                            }
                        }

                        // check left for diagonals
                        if has_left && has_above {
                            let top_left = matrix[v.row - 1][v.col - 1];
                            if is_simbol(top_left) {
                                is_adjacent = true;
                                break;
                            }
                        }

                        if has_left && has_bottom {
                            let bottom_left = matrix[v.row + 1][v.col - 1];
                            if is_simbol(bottom_left) {
                                is_adjacent = true;
                                break;
                            }
                        }

                        if has_right {
                            let right_value = matrix[v.row][v.col + 1];
                            if is_simbol(right_value) {
                                is_adjacent = true;
                                break;
                            }
                        }

                        if has_bottom {
                            let bottom_value = matrix[v.row + 1][v.col];
                            if is_simbol(bottom_value) {
                                is_adjacent = true;
                                break;
                            }
                        }

                        // check right for diagonals
                        if has_right && has_above {
                            let top_right = matrix[v.row - 1][v.col + 1];
                            if is_simbol(top_right) {
                                is_adjacent = true;
                                break;
                            }
                        }

                        if has_right && has_bottom {
                            let bottom_right = matrix[v.row + 1][v.col + 1];
                            if is_simbol(bottom_right) {
                                is_adjacent = true;
                                break;
                            }
                        }
                    }

                    if is_adjacent {
                        let number = current_values
                            .iter()
                            .fold("".to_string(), |acc, c| format!("{}{}", acc, c.value));
                        numbers.push(number.parse().unwrap());
                    }

                    // reset accumulator
                    current_values = Vec::new();
                }
            }
        }

        println!("result: {:?}", numbers.iter().sum::<usize>());
    }

    fn part_2(&self) {
        println!("Part 2 - Result: todo");
    }
}

fn is_simbol(c: char) -> bool {
    !c.is_ascii_digit() && c != '.'
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
