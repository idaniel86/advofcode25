use std::{
    fs::File,
    io::{self, BufRead},
};

const MOVES: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn get_sum(x: usize, y: usize, array: &Vec<Vec<u32>>) -> u32 {
    let y_max = array.len() as i32;
    let x_max = array[0].len() as i32;

    MOVES.iter().fold(0, |acc, (x_move, y_move)| {
        let x_move = x as i32 + x_move;
        let y_move = y as i32 + y_move;

        acc + if 0 <= x_move && x_move < x_max && 0 <= y_move && y_move < y_max {
            array[y_move as usize][x_move as usize]
        } else {
            0
        }
    })
}

fn get_rolls() -> Vec<Vec<u32>> {
    let reader = io::BufReader::new(File::open("inputs/day04.txt").unwrap());

    reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .into_iter()
                .map(|c| match c {
                    '.' => 0,
                    '@' => 1,
                    _ => panic!("unknown character read"),
                })
                .collect::<Vec<u32>>()
        })
        .collect()
}

fn part1() -> u32 {
    let rolls = get_rolls();

    let mut acc = 0u32;
    for (y, row) in rolls.iter().enumerate() {
        for (x, column) in row.iter().enumerate() {
            if *column == 1 && get_sum(x, y, &rolls) < 4 {
                acc += 1;
            }
        }
    }

    acc
}

fn part2() -> u32 {
    let mut rolls = get_rolls();
    let mut rolls_after_remove = rolls.clone();

    let mut acc = 0u32;

    loop {
        let mut rolls_removed = 0;
        for (y, row) in rolls.iter().enumerate() {
            for (x, column) in row.iter().enumerate() {
                if *column == 1 && get_sum(x, y, &rolls) < 4 {
                    rolls_after_remove[y][x] = 0;
                    rolls_removed += 1;
                }
            }
        }
        if rolls_removed == 0 {
            break;
        }

        acc += rolls_removed;
        rolls = rolls_after_remove.clone();
    }

    acc
}

fn main() {
    println!("Part1: rolls of paper removed = {:?}", part1());
    println!("Part2: rolls of paper removed = {:?}", part2());
}
