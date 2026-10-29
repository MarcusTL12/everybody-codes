use std::{cmp::Ordering, collections::BTreeMap};

use crate::{
    Quest,
    QuestResult::{self, Unsigned},
};

pub const PARTS: Quest = [part1, part2, part3];

fn find_order(line: &str) -> u64 {
    let mut n_alloys = 0;

    let ord_lookup: BTreeMap<_, _> = line
        .as_bytes()
        .split(|&c| c == b' ')
        .flat_map(|s| {
            let mut n1: u8 = 0;
            let mut n2: u8 = 0;

            let mut n = &mut n1;

            let mut ord = Ordering::Equal;
            let mut other_ord = Ordering::Equal;

            for &c in s {
                match c {
                    b'0'..=b'9' => *n = 10 * *n + (c - b'0'),
                    b'>' => {
                        n = &mut n2;
                        ord = Ordering::Greater;
                        other_ord = Ordering::Less;
                    }
                    b'<' => {
                        n = &mut n2;
                        ord = Ordering::Less;
                        other_ord = Ordering::Greater;
                    }
                    _ => panic!(),
                }
            }

            n_alloys = n_alloys.max(n1.max(n2));

            [((n1, n2), ord), ((n2, n1), other_ord)]
        })
        .collect();

    let mut alloys: Vec<_> = (1..=n_alloys).collect();

    alloys.sort_by(|&a, &b| ord_lookup[&(a, b)]);

    alloys.into_iter().rev().fold(0, |n, d| 10 * n + d as u64)
}

fn part1(input: String) -> QuestResult {
    let ans = input.lines().map(find_order).sum();

    Unsigned(ans)
}

fn part2(input: String) -> QuestResult {
    todo!("\n{input}")
}

fn part3(input: String) -> QuestResult {
    todo!("\n{input}")
}
