use std::cmp::Reverse;

use crate::{
    Quest,
    QuestResult::{self, Unsigned},
};

pub const PARTS: Quest = [part1, part2, part3];

fn parsebytes<I: Iterator<Item = u8>>(bytes: I) -> u32 {
    bytes.fold(0, |n, d| 10 * n + (d - b'0') as u32)
}

fn parseinput(input: &str) -> impl Iterator<Item = (u32, u32)> {
    input.as_bytes().split(|&c| c == b'\n').map(|l| {
        let mut it = l.split(|&c| c == b' ').step_by(3);
        let id = parsebytes(
            it.next()
                .unwrap()
                .split_once(|&c| c == b'=')
                .unwrap()
                .1
                .iter()
                .cloned(),
        );

        let t = parsebytes(
            it.next()
                .unwrap()
                .split_once(|&c| c == b'=')
                .unwrap()
                .1
                .iter()
                .cloned(),
        );

        (id, t)
    })
}

fn part1(input: String) -> QuestResult {
    let mut record = Vec::new();

    for (id, t) in parseinput(&input) {
        if record.len() < id as usize {
            record.resize(id as usize, 0);
        }

        record[id as usize - 1] += t;
    }

    record.sort();

    let ans: u32 = record
        .into_iter()
        .enumerate()
        .map(|(i, t)| (i as u32 + 1) * t)
        .sum();

    Unsigned(ans.into())
}

fn part2(input: String) -> QuestResult {
    let mut record = Vec::new();

    for (id, t) in parseinput(&input) {
        if record.len() < id as usize {
            record.resize(id as usize, (Reverse(0u32), 0));
        }

        record[id as usize - 1].0.0 += 1;
        record[id as usize - 1].1 += t;
    }

    record.sort();

    let ans: u32 = record
        .into_iter()
        .enumerate()
        .map(|(i, (_, t))| (i as u32 + 1) * t)
        .sum();

    Unsigned(ans.into())
}

fn part3(input: String) -> QuestResult {
    todo!("\n{input}")
}
