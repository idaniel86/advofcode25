use std::collections::HashSet;

#[derive(Debug, serde::Deserialize, PartialEq, Eq, Hash, Clone, Copy)]
struct JunctionBox {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug)]
struct Distance {
    box1: JunctionBox,
    box2: JunctionBox,
    distance: i64,
}

fn get_distances() -> Vec<Distance> {
    let boxes: Vec<JunctionBox> = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path("inputs/day08.txt")
        .unwrap()
        .deserialize()
        .map(|record| record.unwrap())
        .collect();

    let mut distances = vec![];
    for i in 0..boxes.len() - 1 {
        for j in i + 1..boxes.len() {
            distances.push(Distance {
                box1: boxes[i],
                box2: boxes[j],
                distance: ((boxes[i].x - boxes[j].x).pow(2)
                    + (boxes[i].y - boxes[j].y).pow(2)
                    + (boxes[i].z - boxes[j].z).pow(2))
                .isqrt(),
            });
        }
    }
    distances.sort_by(|a, b| a.distance.cmp(&b.distance));
    distances
}

fn part1() -> usize {
    let distances = &get_distances()[..1000];

    let mut circuits: Vec<HashSet<&JunctionBox>> = vec![];

    for Distance { box1, box2, .. } in distances {
        let mut merged = HashSet::new();
        merged.insert(box1);
        merged.insert(box2);

        let mut to_merge = Vec::new();
        for (index, group) in circuits.iter().enumerate() {
            if group.contains(box1) || group.contains(box2) {
                to_merge.push(index);
            }
        }

        for index in to_merge.iter().rev() {
            merged.extend(circuits.swap_remove(*index));
        }

        circuits.push(merged);
    }

    let mut sizes: Vec<usize> = circuits.iter().map(|circuit| circuit.len()).collect();
    sizes.sort_by(|a, b| b.cmp(a));

    sizes[0] * sizes[1] * sizes[2]
}

fn part2() -> usize {
    let distances = &get_distances();

    let mut circuits: Vec<HashSet<&JunctionBox>> = vec![];

    for Distance { box1, box2, .. } in distances {
        let mut merged = HashSet::new();
        merged.insert(box1);
        merged.insert(box2);

        let mut to_merge = Vec::new();
        for (index, group) in circuits.iter().enumerate() {
            if group.contains(box1) || group.contains(box2) {
                to_merge.push(index);
            }
        }

        for index in to_merge.iter().rev() {
            merged.extend(circuits.swap_remove(*index));
        }
        if merged.len() == 1000 {
            return box1.x as usize * box2.x as usize;
        }

        circuits.push(merged);
    }

    0
}

fn main() {
    println!("Part1: {:?}", part1());
    println!("Part2: {:?}", part2());
}
