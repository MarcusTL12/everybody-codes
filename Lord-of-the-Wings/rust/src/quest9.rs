use std::{collections::BTreeMap, mem::swap};

use crate::{
    Quest,
    QuestResult::{self, Unsigned},
};

pub const PARTS: Quest = [part1, part2, part3];

fn grow(split_thrs: u32, limit: u32) -> u32 {
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

fn part1(input: String) -> QuestResult {
    let ans: u32 = input
        .lines()
        .map(|l| {
            let mut parts = l
                .split(", ")
                .skip(1)
                .map(|p| p.split_once('=').unwrap().1.parse().unwrap());

            let split_thrs = parts.next().unwrap();
            let limit = parts.next().unwrap();

            grow(split_thrs, limit)
        })
        .sum();

    Unsigned(ans.into())
}

fn part2(input: String) -> QuestResult {
    todo!("\n{input}")
}

fn part3(input: String) -> QuestResult {
    todo!("\n{input}")
}
