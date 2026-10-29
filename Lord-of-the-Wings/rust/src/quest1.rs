use crate::{
    Quest,
    QuestResult::{self, Unsigned},
};

pub const PARTS: Quest = [part1, part2, part3];

fn isqrt_ceil(x: u64) -> u64 {
    (x as f64).sqrt().ceil() as u64
}

fn part1(input: String) -> QuestResult {
    let tmp: u64 = input
        .split(',')
        .map(|x| x.parse().unwrap())
        .map(|x| isqrt_ceil(x).pow(2) - x)
        .sum();

    Unsigned(tmp)
}

fn part2(input: String) -> QuestResult {
    todo!("\n{input}")
}

fn part3(input: String) -> QuestResult {
    todo!("\n{input}")
}
