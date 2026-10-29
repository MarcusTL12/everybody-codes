use std::{collections::BTreeMap, mem::swap};

use arrayvec::ArrayVec;

use crate::{
    Quest,
    QuestResult::{self, Number},
};

pub const PARTS: Quest = [part1, part2, part3];

type Network<'a> = BTreeMap<&'a [u8], ArrayVec<&'a [u8], 5>>;

fn parse_network(input: &str) -> Network<'_> {
    input
        .as_bytes()
        .split(|&x| x == b'\n')
        .map(|l| {
            let (from, to) = l.split_once(|&x| x == b':').unwrap();

            let to = to.split(|&x| x == b',').collect();

            (from, to)
        })
        .collect()
}

fn propagate_state<'a>(
    network: &Network<'a>,
    state1: &BTreeMap<&'a [u8], u64>,
    state2: &mut BTreeMap<&'a [u8], u64>,
) {
    state2.clear();

    for (from, v) in state1 {
        for to in &network[from] {
            if let Some(c) = state2.get_mut(to) {
                *c += v;
            } else {
                state2.insert(*to, *v);
            }
        }
    }
}

fn calculate_population<'a>(
    network: &Network<'a>,
    state1: &mut BTreeMap<&'a [u8], u64>,
    state2: &mut BTreeMap<&'a [u8], u64>,
    init_termite: &'a [u8],
    steps: usize,
) -> u64 {
    state1.clear();
    state1.insert(init_termite, 1);

    for _ in 0..steps {
        propagate_state(network, state1, state2);
        swap(state1, state2);
    }

    state1.values().sum()
}

fn part1(input: String) -> QuestResult {
    let network = parse_network(&input);

    let mut state1 = BTreeMap::new();
    let mut state2 = BTreeMap::new();

    Number(
        calculate_population(&network, &mut state1, &mut state2, b"A", 4)
            as i64,
    )
}

fn part2(input: String) -> QuestResult {
    let network = parse_network(&input);

    let mut state1 = BTreeMap::new();
    let mut state2 = BTreeMap::new();

    Number(
        calculate_population(&network, &mut state1, &mut state2, b"Z", 10)
            as i64,
    )
}

fn part3(input: String) -> QuestResult {
    let network = parse_network(&input);

    let mut state1 = BTreeMap::new();
    let mut state2 = BTreeMap::new();

    let (min, max) = network
        .keys()
        .map(|&k| {
            calculate_population(&network, &mut state1, &mut state2, k, 20)
        })
        .fold((u64::MAX, u64::MIN), |(min, max), x| {
            (min.min(x), max.max(x))
        });

    Number((max - min) as i64)
}
