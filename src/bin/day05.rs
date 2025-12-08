use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let reader = io::BufReader::new(File::open("inputs/day05.txt").unwrap());

    let mut iter = reader.lines().into_iter();

    // get fresh ingredient ranges
    let mut ranges: Vec<(u64, u64)> = iter
        .by_ref()
        .map_while(|line| {
            line.unwrap()
                .split_once("-")
                .and_then(|(start, end)| Some((start.parse().unwrap(), end.parse().unwrap())))
        })
        .collect();

    // get ingredients
    let ingredients: Vec<u64> = iter
        .by_ref()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    let mut fresh_ingredients = HashSet::new();

    for ingredient in ingredients.iter() {
        for (start, end) in ranges.iter() {
            if (start..=end).contains(&ingredient) {
                fresh_ingredients.insert(ingredient);
            }
        }
    }

    let mut merged_ranges: HashSet<(u64, u64)> = ranges.clone().into_iter().collect();
    let mut merged = true;

    while merged {
        let mut iter = ranges.iter();
        merged = false;

        while let Some((start1, end1)) = iter.next() {
            let mut iter_cloned = iter.clone();

            while let Some((start2, end2)) = iter_cloned.next() {
                if start1.max(start2) <= end1.min(end2) {
                    merged_ranges.remove(&(*start1, *end1));
                    merged_ranges.remove(&(*start2, *end2));
                    merged_ranges.insert((*start1.min(start2), *end1.max(end2)));
                    merged = true;
                }
            }
        }

        ranges = merged_ranges.clone().into_iter().collect();
    }

    let total_fresh_ingredients = merged_ranges
        .iter()
        .fold(0, |acc, (start, end)| acc + end - start + 1);

    println!(
        "Part1: available fresh ingredients = {:?}",
        fresh_ingredients.len()
    );
    println!(
        "Part2: total fresh ingredients = {:?}",
        total_fresh_ingredients
    );
}
