use std::{collections::HashSet, iter::Step};

use num_traits::PrimInt;

use crate::{Quest, QuestResult};

pub const PARTS: Quest = [part1, part2, part3];

fn clap_into_col<T: PrimInt + Step>(col: &mut Vec<T>, clapper: T) {
    let mut i = 0;
    let mut d = 1;

    for _ in T::one()..clapper {
        if d == 1 && i < col.len() - 1 {
            i += 1;
        } else if d == -1 && i > 0 {
            i -= 1;
        } else {
            d = -d;
        }
    }

    if d == -1 {
        i += 1;
    }

    col.insert(i, clapper);
}

fn part1(input: String) -> QuestResult {
    let mut cols: [Vec<u8>; 4] = [const { Vec::new() }; 4];

    for l in input.split('\n') {
        for (col, x) in cols.iter_mut().zip(l.split_ascii_whitespace()) {
            col.push(x.parse().unwrap());
        }
    }

    for i in 0..10 {
        let clapper = cols[i % 4].remove(0);
        clap_into_col(&mut cols[(i + 1) % 4], clapper);
    }

    QuestResult::Number(
        cols.iter()
            .map(|c| c[0])
            .fold(0, |n, d| 10 * n + (d as i64)),
    )
}

fn part2(input: String) -> QuestResult {
    let mut cols: [Vec<u8>; 4] = [const { Vec::new() }; 4];

    for l in input.split('\n') {
        for (col, x) in cols.iter_mut().zip(l.split_ascii_whitespace()) {
            col.push(x.parse().unwrap());
        }
    }

    let mut n_shouts = vec![0u16; 1_000_000_000];

    let mut i = 0;

    QuestResult::Number(loop {
        let clapper = cols[i % 4].remove(0);
        clap_into_col(&mut cols[(i + 1) % 4], clapper);

        let n = cols
            .iter()
            .map(|c| c[0])
            .fold(0, |n, d| 100 * n + (d as usize));

        n_shouts[n] += 1;
        i += 1;

        if n_shouts[n] == 2024 {
            break n * i;
        }
    } as i64)
}

fn part3(input: String) -> QuestResult {
    let mut cols: [Vec<u16>; 4] = [const { Vec::new() }; 4];

    for l in input.split('\n') {
        for (col, x) in cols.iter_mut().zip(l.split_ascii_whitespace()) {
            col.push(x.parse().unwrap());
        }
    }

    let mut max_seen = 0;
    let mut seen = HashSet::new();

    for i in 0..usize::MAX {
        let clapper = cols[i % 4].remove(0);
        clap_into_col(&mut cols[(i + 1) % 4], clapper);

        if seen.contains(&cols) {
            break;
        }

        max_seen = max_seen.max(
            cols.iter()
                .map(|c| c[0])
                .fold(0, |n, d| 10000 * n + (d as i64)),
        );

        seen.insert(cols.clone());
    }

    QuestResult::Number(max_seen)
}
