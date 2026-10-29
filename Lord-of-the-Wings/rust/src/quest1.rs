use crate::{
    Quest,
    QuestResult::{self, Unsigned},
};

pub const PARTS: Quest = [part1, part2, part3];

fn part1(input: String) -> QuestResult {
    let ans: u64 = input
        .split(',')
        .map(|x| x.parse().unwrap())
        .map(|x: u64| ((x as f64).sqrt().ceil() as u64).pow(2) - x)
        .sum();

    Unsigned(ans)
}

fn part2(input: String) -> QuestResult {
    let ans: u64 = input
        .split(',')
        .map(|x| x.parse().unwrap())
        .map(|x: u64| if x <= 8 { 8 - x } else { 4 * x.div_ceil(4) - x })
        .sum();

    Unsigned(ans)
}

fn part3(input: String) -> QuestResult {
    let ans: u64 = input
        .split(',')
        .map(|x| x.parse().unwrap())
        .map(|x: u64| {
            if x <= 8 {
                8 - x
            } else {
                let n = ((((x + 1) as f64).sqrt() - 1.0) * 0.5).ceil() as u64;
                let y = (2 * n + 1).pow(2) - 1;

                y - x
            }
        })
        .sum();

    Unsigned(ans)
}
