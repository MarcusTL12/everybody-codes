use std::collections::{BTreeMap, BTreeSet};

use crate::{
    Quest,
    QuestResult::{self, Unsigned},
};

pub const PARTS: Quest = [part1, part2, part3];

fn read_id(data: &[u8], i: usize) -> [u8; 4] {
    let j = i * 5;
    unsafe { [0, 1, 2, 3].map(|k| *data.get_unchecked(j + k)) }
}

fn dfs(
    ingredients: &BTreeMap<[u8; 4], (usize, &[u8])>,
    memo: &mut BTreeSet<[u8; 4]>,
    k: [u8; 4],
) {
    if memo.contains(&k) {
        return;
    }

    memo.insert(k);

    let Some(&(n, l)) = ingredients.get(&k) else {
        return;
    };

    for i in 1..=n {
        let id = read_id(l, i);

        dfs(ingredients, memo, id);
    }
}

fn part1(input: String) -> QuestResult {
    let map: BTreeMap<_, _> = input
        .as_bytes()
        .split(|&c| c == b'\n')
        .map(|l| {
            let k = read_id(l, 0);

            (k, (l.len() / 5, l))
        })
        .collect();

    let mut memo = BTreeSet::new();

    dfs(&map, &mut memo, *b"NULL");

    Unsigned((memo.len() - 1) as u64)
}

fn part2(input: String) -> QuestResult {
    todo!("\n{input}")
}

fn part3(input: String) -> QuestResult {
    todo!("\n{input}")
}
