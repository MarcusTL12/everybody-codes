use crate::{Quest, QuestResult};

pub const PARTS: Quest = [part1, part2, part3];

fn map_to_potions(x: u8) -> u8 {
    match x {
        b'A' => 0,
        b'B' => 1,
        b'C' => 3,
        b'D' => 5,
        _ => 0,
    }
}

fn part1(input: String) -> QuestResult {
    let ans = input.bytes().map(|c| map_to_potions(c) as i64).sum();

    QuestResult::Number(ans)
}

fn part2(input: String) -> QuestResult {
    let ans = input
        .as_bytes()
        .as_chunks::<2>()
        .0
        .iter()
        .map(|&(mut x)| {
            x.sort();
            match x {
                [a, b'x'] => map_to_potions(a) as i64,
                x => {
                    x.iter().map(|&c| map_to_potions(c) as i64).sum::<i64>() + 2
                }
            }
        })
        .sum();

    QuestResult::Number(ans)
}

fn part3(input: String) -> QuestResult {
    let ans = input
        .as_bytes()
        .as_chunks::<3>()
        .0
        .iter()
        .map(|&(mut x)| {
            x.sort();
            match x {
                [a, b'x', b'x'] => map_to_potions(a) as i64,
                [a, b, b'x'] => {
                    (map_to_potions(a) + map_to_potions(b) + 2) as i64
                }
                x => {
                    x.iter().map(|&c| map_to_potions(c) as i64).sum::<i64>() + 6
                }
            }
        })
        .sum();

    QuestResult::Number(ans)
}
