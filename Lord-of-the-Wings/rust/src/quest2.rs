use arrayvec::ArrayVec;
use bitarray::BitArray;

use crate::{
    Quest,
    QuestResult::{self, Unsigned},
};

pub const PARTS: Quest = [part1, part1, part1];

fn find_order(line: &str) -> u64 {
    const N: usize = 1;
    const MAX_ALLOYS: usize = N * usize::BITS as usize;
    type U = u8;

    assert!(U::MAX as usize + 1 >= MAX_ALLOYS);

    let mut graph = [BitArray::<N>::new(); MAX_ALLOYS];

    let mut num_alloys = 0;

    for s in line.as_bytes().split(|&c| c == b' ') {
        let mut n1: U = 0;
        let mut n2: U = 0;

        let mut n = &mut n1;

        let mut order = 0;

        for &c in s {
            match c {
                b'0'..=b'9' => *n = 10 * *n + (c - b'0'),
                b'>' | b'<' => {
                    n = &mut n2;
                    order = c;
                }
                _ => panic!(),
            }
        }

        if order == b'>' {
            std::mem::swap(&mut n1, &mut n2);
        }

        num_alloys = num_alloys.max(n1.max(n2));

        // Makes alloys better suited for 0 indexing
        n1 -= 1;
        n2 -= 1;

        graph[n1 as usize].set(n2 as usize, true);
    }

    let mut seen = BitArray::<N>::new();
    let mut order = ArrayVec::<U, MAX_ALLOYS>::new();

    fn dfs(
        graph: &[BitArray<N>; MAX_ALLOYS],
        seen: &mut BitArray<N>,
        order: &mut ArrayVec<U, MAX_ALLOYS>,
        alloy: U,
    ) {
        if seen[alloy as usize] {
            return;
        }

        for other_alloy in graph[alloy as usize].trues_iter() {
            dfs(graph, seen, order, other_alloy as U);
        }

        seen.set(alloy as usize, true);
        order.push(alloy);
    }

    for alloy in 0..num_alloys {
        dfs(&graph, &mut seen, &mut order, alloy);
    }

    order.into_iter().fold(0, |n, d| 10 * n + (d as u64) + 1)
}

fn part1(input: String) -> QuestResult {
    let ans = input.lines().map(find_order).sum();

    Unsigned(ans)
}
