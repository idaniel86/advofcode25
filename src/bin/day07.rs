use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let reader = io::BufReader::new(File::open("inputs/day07.txt").unwrap());

    let lines = reader
        .lines()
        .into_iter()
        .collect::<Vec<Result<String, io::Error>>>();

    let mut iter = lines.into_iter();

    let mut beam_line: Vec<u64> = iter
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .map(|c| match c {
            '.' => 0,
            'S' => 1,
            _ => panic!("unexpected quantum space"),
        })
        .collect();

    let mut beam_splits = 0u64;

    while let Some(Ok(line)) = iter.next() {
        for (index, c) in line.chars().enumerate() {
            let paths = beam_line[index];

            if c == '^' && paths != 0 {
                // we have a split
                beam_splits += 1;
                beam_line[index - 1] += paths;
                beam_line[index] = 0;
                beam_line[index + 1] += paths;
            }
        }
    }

    println!("Part1: beams splits = {:?}", beam_splits);
    println!("Part2: timelines = {:?}", beam_line.iter().sum::<u64>());
}
