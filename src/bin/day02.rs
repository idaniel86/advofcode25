use std::{
    fs::File,
    io::{self, BufRead},
    usize,
};

fn get_ranges() -> Vec<(usize, usize)> {
    let reader = io::BufReader::new(File::open("inputs/day02.txt").unwrap());

    let ranges: Vec<_> = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .split(",")
                .map(|range| {
                    let (start, end) = range.split_once("-").unwrap();
                    (
                        start.parse::<usize>().unwrap(),
                        end.parse::<usize>().unwrap(),
                    )
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    ranges
}

fn part1() -> usize {
    let mut acc = 0usize;

    for (start, end) in get_ranges() {
        for value in start..=end {
            let digits = value.to_string().len();

            if digits % 2 == 0 {
                let decimals = (digits / 2) as u32;
                let pattern = 10usize.pow(decimals);

                if value % pattern == value / pattern {
                    acc += value;
                }
            }
        }
    }

    acc
}

fn main() -> io::Result<()> {
    println!("Part1: invalid IDs sum = {:?}", part1());
    Ok(())
}
