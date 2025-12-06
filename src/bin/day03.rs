use std::{
    fs::File,
    io::{self, BufRead},
};

fn joltage(batteries_count: usize) -> u64 {
    let reader = io::BufReader::new(File::open("inputs/day03.txt").unwrap());

    let joltage = reader
        .lines()
        .map(|line| line.unwrap())
        .into_iter()
        .fold(0, |acc, bank| {
            let mut batteries = vec![];
            let mut start = 0;

            for end in (bank.len() - batteries_count)..bank.len() {
                let range = &bank.as_bytes()[start..=end];
                let max = range.iter().max().unwrap();
                start += range.iter().position(|x| x == max).unwrap() + 1;
                batteries.push(*max);
            }

            let batteries: u64 = String::from_utf8(batteries).unwrap().parse().unwrap();

            acc + batteries
        });

    joltage
}

fn main() {
    println!("Part1: joltage = {:?}", joltage(2));
    println!("Part2: joltage = {:?}", joltage(12));
}
