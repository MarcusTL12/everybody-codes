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

fn dfs<I: Iterator<Item = [u8; 4]>, F: Copy + Fn([u8; 4]) -> I>(
    ingredients: F,
    memo: &mut BTreeSet<[u8; 4]>,
    k: [u8; 4],
) {
    if memo.contains(&k) {
        return;
    }

    memo.insert(k);

    for id in ingredients(k) {
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

    dfs(
        |k| {
            map.get(&k)
                .into_iter()
                .flat_map(|&(n, l)| (1..=n).map(|i| read_id(l, i)))
        },
        &mut memo,
        *b"NULL",
    );

    Unsigned((memo.len() - 1) as u64)
}

fn part2(input: String) -> QuestResult {
    let mut map = BTreeMap::new();

    for l in input.as_bytes().split(|&c| c == b'\n') {
        let k = read_id(l, 0);

        for i in 1..=l.len() / 5 {
            let v = read_id(l, i);

            map.entry(v).or_insert(Vec::new()).push(k);
        }
    }

    let mut memo = BTreeSet::new();

    dfs(
        |k| map.get(&k).into_iter().flat_map(|v| v.iter().cloned()),
        &mut memo,
        *b"NULL",
    );

    Unsigned((memo.len() - 1) as u64)
}

fn part3(input: String) -> QuestResult {
    todo!("\n{input}")
}
