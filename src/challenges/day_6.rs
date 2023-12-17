use std::fs;

pub struct Day6 {}

impl Day6 {
    pub fn new() -> Self {
        Self {}
    }

    // See: https://adventofcode.com/2023/day/5
    fn part_1(&self) -> usize {
        let races = self.parse();
        races
            .iter()
            .fold(1, |acc, race| acc * race.ways_to_beat_the_record())
    }

    fn part_2(&self) -> usize {
        println!("TODO!");
        1
    }

    fn parse(&self) -> Vec<Race> {
        let contents = fs::read_to_string("inputs/day_6.txt").unwrap();
        let mut times: Vec<usize> = Vec::new();
        let mut distances: Vec<usize> = Vec::new();
        let mut races: Vec<Race> = Vec::new();

        let (times_line, distances_line) = (
            contents.split('\n').collect::<Vec<&str>>()[0],
            contents.split('\n').collect::<Vec<&str>>()[1],
        );

        for value in Self::parse_line(times_line) {
            let time = value.trim().parse::<usize>().unwrap();
            times.push(time);
        }

        for value in Self::parse_line(distances_line) {
            let distance = value.trim().parse::<usize>().unwrap();
            distances.push(distance);
        }

        for i in 0..times.len() {
            let distance = distances[i];
            let time = times[i];
            races.push(Race::new(time, distance));
        }

        races
    }

    fn parse_line(input: &str) -> Vec<String> {
        input
            .split(':')
            .skip(1)
            .map(|l| l.trim().to_string())
            .collect::<Vec<String>>()
            .join("")
            .split(' ')
            .filter(|l| !l.is_empty())
            .map(|l| l.to_string())
            .collect::<Vec<String>>()
    }
}

#[derive(Debug)]
struct Race {
    time: usize,
    record_distance: usize,
}

impl Race {
    fn new(time: usize, distance: usize) -> Self {
        Self {
            time,
            record_distance: distance,
        }
    }

    fn ways_to_beat_the_record(&self) -> usize {
        (0..self.time)
            .map(|i| {
                let mm_per_second = i;
                mm_per_second * (self.time - i)
            })
            .filter(|distance| *distance > self.record_distance)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let day_6 = Day6::new();
        day_6.parse();
        let result = day_6.part_1();
        assert_eq!(result, 500346);
    }

    #[ignore = "Not implemented yet"]
    #[test]
    fn part_2() {
        let day_6 = Day6::new();
        let result = day_6.part_2();
        assert_eq!(result, 1);
    }
}
