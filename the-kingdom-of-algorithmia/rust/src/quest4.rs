use crate::{Quest, QuestResult};

pub const PARTS: Quest = [part1, part1, part3];

fn part1(input: String) -> QuestResult {
    let mut min = u64::MAX;
    let mut s = 0;
    let mut n = 0;

    for l in input.split('\n') {
        n += 1;

        let x = l.parse().unwrap();

        min = min.min(x);
        s += x;
    }

    QuestResult::Number((s - n * min) as i64)
}

fn part3(input: String) -> QuestResult {
    let numbers: Vec<i64> =
        input.split('\n').map(|s| s.parse().unwrap()).collect();

    QuestResult::Number(
        numbers
            .iter()
            .map(|target_x| numbers.iter().map(|x| (x - target_x).abs()).sum())
            .min()
            .unwrap(),
    )
}
