// use std::collections::{BTreeMap, BTreeSet};

use psst::{PsstMap, set::PsstSet};

use crate::{
    Quest,
    QuestResult::{self, Unsigned},
};

pub const PARTS: Quest = [part1, part2, part3];

type Map<K, V> = PsstMap<K, V>;
type Set<T> = PsstSet<T>;

fn read_id(data: &[u8], i: usize) -> [u8; 4] {
    let j = i * 5;
    unsafe { [0, 1, 2, 3].map(|k| *data.get_unchecked(j + k)) }
}

fn dfs<I: Iterator<Item = [u8; 4]>, F: Copy + Fn([u8; 4]) -> I>(
    ingredients: F,
    memo: &mut Set<[u8; 4]>,
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
    let map: Map<_, _> = input
        .as_bytes()
        .split(|&c| c == b'\n')
        .map(|l| {
            let k = read_id(l, 0);

            (k, (l.len() / 5, l))
        })
        .collect();

    let mut memo = Set::new();

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
    let mut map = Map::new();

    for l in input.as_bytes().split(|&c| c == b'\n') {
        let k = read_id(l, 0);

        for i in 1..=l.len() / 5 {
            let v = read_id(l, i);

            map.entry(v).or_insert(Vec::new()).push(k);
        }
    }

    let mut memo = Set::new();

    dfs(
        |k| map.get(&k).into_iter().flat_map(|v| v.iter().cloned()),
        &mut memo,
        *b"NULL",
    );

    Unsigned((memo.len() - 1) as u64)
}

fn dfs2<'a, I: Iterator<Item = &'a [u8]>, F: Copy + Fn(&'a [u8]) -> I>(
    ingredients: F,
    memo: &mut Map<&'a [u8], u8>,
    k: &'a [u8],
    i: usize,
) {
    if let Some(x) = memo.get(&k)
        && x & 1 << i != 0
    {
        return;
    }

    *memo.entry(k).or_insert(0) |= 1 << i;

    for id in ingredients(k) {
        dfs2(ingredients, memo, id, i);
    }
}

fn part3(input: String) -> QuestResult {
    let mut map = Map::new();

    for l in input.as_bytes().split(|&c| c == b'\n') {
        let (k, vs) = l.split_once(|&c| c == b':').unwrap();

        for v in vs.split(|&c| c == b',') {
            map.entry(v).or_insert(Vec::new()).push(k);
        }
    }

    let mut memo = Map::new();

    for (i, null) in [b"NULLv1", b"NULLv2", b"NULLv3"].into_iter().enumerate() {
        dfs2(
            |k| map.get(&k).into_iter().flat_map(|v| v.iter().cloned()),
            &mut memo,
            null,
            i,
        );
    }

    let ans = memo.values().filter(|x| x.count_ones() >= 2).count();

    Unsigned(ans as u64)
}
