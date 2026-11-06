use std::{collections::BTreeMap, mem::swap};

use rayon::{iter::ParallelIterator, str::ParallelString};

use crate::{
    Quest,
    QuestResult::{self, Unsigned},
};

pub const PARTS: Quest = [part1, part2, part2];

fn grow(split_thrs: u64, limit: u64) -> u64 {
    let mut queue1 = BTreeMap::new();
    let mut queue2 = BTreeMap::new();

    queue1.insert(0, 1);

    let mut i = 0;

    let mut joints = 0;

    loop {
        i += 1;

        let mut elements = 0;

        while let Some((l, n)) = queue1.pop_first() {
            elements += (l + 1) * n;
            if l + 1 < split_thrs {
                *queue2.entry(l + 1).or_insert(0) += n;
            } else {
                let lhalf = l / 2;
                *queue2.entry(lhalf).or_insert(0) += 2 * n;
                *queue2.entry(1).or_insert(0) += n;
                joints += n;
            }
        }

        elements += joints;

        if elements >= limit {
            break i;
        }

        swap(&mut queue1, &mut queue2);
    }
}

fn grow_input<F: Fn(u64, u64) -> u64 + Sync>(input: String, grow: F) -> u64 {
    input
        .par_lines()
        .map(|l| {
            let mut parts = l
                .split(", ")
                .skip(1)
                .map(|p| p.split_once('=').unwrap().1.parse().unwrap());

            let split_thrs = parts.next().unwrap();
            let limit = parts.next().unwrap();

            grow(split_thrs, limit)
        })
        .sum()
}

fn part1(input: String) -> QuestResult {
    Unsigned(grow_input(input, grow))
}

fn grow2(split_thrs: u64, limit: u64) -> u64 {
    let mut queue1 = BTreeMap::new();
    let mut queue2 = BTreeMap::new();

    queue1.insert(0, 1);

    let mut i = 0;
    let mut leaves = 1;

    if limit == 1 {
        return 1;
    }

    loop {
        let max_len = queue1.keys().max().unwrap();
        let timestep = split_thrs - max_len;

        i += timestep;

        while let Some((l, n)) = queue1.pop_first() {
            if l + timestep < split_thrs {
                *queue2.entry(l + timestep).or_insert(0) += n;
            } else {
                let lhalf = (l + timestep) / 2;
                *queue2.entry(lhalf).or_insert(0) += 2 * n;
                *queue2.entry(1).or_insert(0) += n;
                leaves += n;
            }
        }

        if leaves >= limit {
            break i;
        }

        swap(&mut queue1, &mut queue2);
    }
}

fn part2(input: String) -> QuestResult {
    Unsigned(grow_input(input, grow2))
}
