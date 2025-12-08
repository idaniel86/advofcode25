use std::{
    fs::File,
    io::{self, BufRead},
    iter::zip,
};

fn part1() -> u64 {
    let reader = io::BufReader::new(File::open("inputs/day06.txt").unwrap());

    let cells: Vec<Vec<String>> = reader
        .lines()
        .map(|row| {
            row.unwrap()
                .split_ascii_whitespace()
                .map(|collumn| collumn.to_owned())
                .collect()
        })
        .collect();

    let mut iter = cells.iter().rev();

    let operations = iter.next().unwrap();
    let init = operations
        .iter()
        .map(|operation| match operation.as_str() {
            "+" => 0,
            "*" => 1,
            _ => panic!("unsupported operation"),
        })
        .collect::<Vec<u64>>();

    iter.fold(init, |acc, row| {
        zip(operations, zip(acc, row))
            .map(|(operation, (acc, value))| match operation.as_str() {
                "+" => acc + value.parse::<u64>().unwrap(),
                "*" => acc * value.parse::<u64>().unwrap(),
                _ => panic!("unsupported operation"),
            })
            .collect()
    })
    .iter()
    .sum()
}

fn part2() -> u64 {
    let reader = io::BufReader::new(File::open("inputs/day06.txt").unwrap());

    let mut lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let init = lines
        .pop()
        .unwrap()
        .split_ascii_whitespace()
        .map(|collumn| collumn.to_owned())
        .rev()
        .map(|operation| match operation.as_str() {
            "+" => 0,
            "*" => 1,
            _ => panic!("unsupported operation"),
        })
        .collect::<Vec<u64>>();

    let mut transposed: Vec<Vec<u64>> = vec![];
    let mut group: Vec<u64> = vec![];
    for _ in (0..lines[0].len()).rev() {
        let line = (0..lines.len())
            .map(|row| lines[row].pop().unwrap())
            .collect::<String>()
            .trim()
            .to_owned();

        if line.is_empty() {
            transposed.push(group);
            group = vec![];
        } else {
            group.push(line.parse().unwrap());
        }
    }
    transposed.push(group);

    zip(init, transposed).fold(0, |acc, (init, values)| {
        let value = values.iter().fold(init, |acc, value| match init {
            0 => acc + *value,
            1 => acc * *value,
            _ => panic!("unsupported operation"),
        });
        acc + value
    })
}

fn main() {
    println!("Part1: grand total: {:?}", part1());
    println!("Part2: grand total: {:?}", part2());
}
