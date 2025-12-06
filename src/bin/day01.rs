use std::{
    fs::File,
    io::{self, BufRead},
};

const INIT_POSITION: i32 = 50;
const DIALS: i32 = 100;

fn get_distances() -> Vec<i32> {
    let reader = io::BufReader::new(File::open("inputs/day01.txt").unwrap());

    reader
        .lines()
        .map(|line| {
            let line = line.expect("expected a string line");
            let (direction, distance) = line.split_at(1);
            let distance = distance
                .parse::<i32>()
                .expect("expected a numeric distance");

            let distance = match direction {
                "L" => -distance,
                "R" => distance,
                _ => panic!("unexpected rotation direction"),
            };
            distance
        })
        .collect()
}

fn part1() -> u32 {
    let (_, password) = get_distances().into_iter().fold(
        (INIT_POSITION, 0),
        |(mut position, mut clicks), distance| {
            position = (position + distance) % DIALS;
            if position == 0 {
                clicks += 1;
            }

            (position, clicks)
        },
    );

    password
}

fn part2() -> u32 {
    let (_, password) = get_distances().into_iter().fold(
        (INIT_POSITION, 0u32),
        |(mut position, mut clicks), distance| {
            for _ in 0..distance.abs() {
                position = (position + distance.signum() + DIALS) % DIALS;
                if position == 0 {
                    clicks += 1;
                }
            }

            (position, clicks)
        },
    );

    password
}

fn main() {
    println!("Part1: password = {:?}", part1());
    println!("Part2: password = {:?}", part2());
}
