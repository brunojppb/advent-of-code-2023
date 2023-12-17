use rayon::prelude::*;
use std::fs;
use std::time::Instant;

pub struct Day5 {}

impl Day5 {
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
    fn part_1(&self) {
        let almanac = self.parse();

        println!("Part 1 - Result: {:#?}", almanac.lowest_location());
    }

    fn part_2(&self) {
        let almanac = self.parse();
        println!("Part 2 - Result: {:?}", almanac.lowest_location_in_pairs());
    }

    fn parse(&self) -> Almanac {
        let contents = fs::read_to_string("inputs/day_5.txt").unwrap();
        Almanac::new(&contents)
    }
}

struct Almanac {
    seeds: Vec<usize>,
    maps: Vec<SourceToDest>,
}

impl Almanac {
    fn new(content: &str) -> Self {
        let mut seeds: Vec<usize> = Vec::new();
        let mut maps: Vec<SourceToDest> = Vec::new();
        let mut acc: Vec<MapRange> = Vec::new();

        for (index, line) in content.lines().enumerate() {
            if index == 0 {
                seeds = line.trim().split(": ").collect::<Vec<&str>>()[1]
                    .trim()
                    .split(' ')
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect();
                continue;
            }

            let next_char = line.chars().next();

            if next_char.is_some_and(|c| c.is_ascii_digit()) {
                let map_range = MapRange::new(line);
                acc.push(map_range);
                continue;
            }

            if next_char.is_some_and(|c| c.is_alphabetic() || c == '\n') && !acc.is_empty() {
                let s = SourceToDest::new(acc);
                maps.push(s);
                acc = Vec::new();
            }
        }

        if !acc.is_empty() {
            let s = SourceToDest::new(acc);
            maps.push(s);
        }

        Self { seeds, maps }
    }

    fn lowest_location(&self) -> usize {
        let mut locations: Vec<usize> = Vec::with_capacity(self.seeds.len());

        for seed in &self.seeds {
            let mut temp_position = *seed;

            for map in &self.maps {
                for map_range in &map.ranges {
                    let range = map_range.source..=(map_range.source + map_range.range);
                    if range.contains(&temp_position) {
                        let next_position =
                            map_range.destination + (temp_position - map_range.source);
                        temp_position = next_position;
                        break;
                    }
                }
            }
            locations.push(temp_position);
        }

        *locations.iter().min().unwrap()
    }

    fn lowest_location_in_pairs(&self) -> usize {
        let small_values = self
            .seeds
            .par_iter()
            .enumerate()
            .fold(
                || {
                    let acc: Vec<usize> = Vec::new();
                    acc
                },
                |mut acc, (index, seed_or_range)| {
                    if index % 2 != 0 {
                        return acc;
                    }

                    let mut smallest_location: Option<usize> = None;

                    // println!(
                    //     "index {}: seed {} with range {}",
                    //     index,
                    //     seed_or_range,
                    //     self.seeds[index + 1]
                    // );

                    for seed in *seed_or_range..(seed_or_range + self.seeds[index + 1]) {
                        let mut temp_position = seed;
                        // println!("Checking seed {}", seed);

                        for map in &self.maps {
                            for map_range in &map.ranges {
                                let range = map_range.source..(map_range.source + map_range.range);
                                if range.contains(&temp_position) {
                                    let next_position =
                                        map_range.destination + (temp_position - map_range.source);
                                    temp_position = next_position;
                                    break;
                                }
                            }
                        }

                        if let Some(loc) = smallest_location {
                            if temp_position < loc {
                                smallest_location = Some(temp_position);
                            }
                        } else {
                            smallest_location = Some(temp_position);
                        }
                    }

                    acc.push(smallest_location.unwrap());
                    acc
                },
            )
            .reduce(
                || {
                    let acc: Vec<usize> = Vec::new();
                    acc
                },
                |mut acc, next| {
                    acc.extend(next);
                    acc
                },
            );

        let result = small_values.iter().fold(usize::MAX, |acc, next| {
            if *next < acc {
                return *next;
            }
            acc
        });

        result
    }
}

#[derive(Debug)]
struct MapRange {
    source: usize,
    destination: usize,
    range: usize,
}

impl MapRange {
    fn new(input: &str) -> Self {
        let values: Vec<&str> = input.split(' ').collect();
        let (destination, source, range) = (
            values[0].parse::<usize>().unwrap(),
            values[1].parse::<usize>().unwrap(),
            values[2].parse::<usize>().unwrap(),
        );

        Self {
            source,
            destination,
            range,
        }
    }
}

#[derive(Debug)]
struct SourceToDest {
    ranges: Vec<MapRange>,
}

impl SourceToDest {
    fn new(ranges: Vec<MapRange>) -> Self {
        Self { ranges }
    }
}
