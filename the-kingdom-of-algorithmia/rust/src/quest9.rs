use std::{cmp::Reverse, collections::BTreeSet};

use priority_queue::PriorityQueue;
use rayon::{iter::ParallelIterator, str::ParallelString};
use z3::{Optimize, ast::Int};

use crate::{
    Quest,
    QuestResult::{self, Number},
};

pub const PARTS: Quest = [part1, part2, part3];

fn find_min_number_of_stamps(stamps: &[u32], target: u32) -> u32 {
    let maxstamp = stamps.iter().max().unwrap();
    let h = |x: u32| x / maxstamp;

    let mut queue = PriorityQueue::new();
    queue.push(target, Reverse(h(target)));
    let mut seen = BTreeSet::new();

    while let Some((rest, Reverse(d))) = queue.pop() {
        let n = d - h(rest);

        if rest == 0 {
            return n;
        }

        seen.insert(rest);

        for &stamp in stamps.iter().rev() {
            if stamp <= rest && !seen.contains(&(rest - stamp)) {
                queue.push_increase(
                    rest - stamp,
                    Reverse(n + 1 + h(rest - stamp)),
                );
            }
        }
    }

    panic!()
}

fn part1(input: String) -> QuestResult {
    let ans: u32 = input
        .split('\n')
        .map(|x| x.parse().unwrap())
        .map(|x| find_min_number_of_stamps(&[1, 3, 5, 10], x))
        .sum();

    Number(ans as i64)
}

fn part2(input: String) -> QuestResult {
    let ans: u32 = input
        .split('\n')
        .map(|x| x.parse().unwrap())
        .map(|x| {
            find_min_number_of_stamps(&[1, 3, 5, 10, 15, 16, 20, 24, 25, 30], x)
        })
        .sum();

    Number(ans as i64)
}

fn find_best_split(target: u32) -> u64 {
    const STAMPS: &[u32] = &[
        1, 3, 5, 10, 15, 16, 20, 24, 25, 30, 37, 38, 49, 50, 74, 75, 100, 101,
    ];

    let left_coeffs: Vec<_> = STAMPS
        .iter()
        .map(|&s| Int::fresh_const(&format!("l_{s}")))
        .collect();

    let right_coeffs: Vec<_> = STAMPS
        .iter()
        .map(|&s| Int::fresh_const(&format!("r_{s}")))
        .collect();

    let left_sum: Int =
        left_coeffs.iter().zip(STAMPS).map(|(c, &s)| c * s).sum();

    let right_sum: Int =
        right_coeffs.iter().zip(STAMPS).map(|(c, &s)| c * s).sum();

    let tot_sum = left_sum + right_sum;

    let left_n: Int = left_coeffs.iter().sum();
    let right_n: Int = right_coeffs.iter().sum();

    let tot_n = &left_n + &right_n;

    let o = Optimize::new();

    o.assert(tot_sum.eq(target));
    o.assert((&left_n - &right_n).le(100));
    o.assert((right_n - left_n).le(100));

    for c in left_coeffs.iter().chain(&right_coeffs) {
        o.assert(c.ge(0));
    }

    o.minimize(&tot_n);
    o.check(&[]);

    // dbg!(o.get_model().unwrap()); // To check which inputs solved

    o.get_lower(0).unwrap().as_int().unwrap().as_u64().unwrap()
}

fn part3(input: String) -> QuestResult {
    let ans: u64 = input
        .par_split('\n')
        .map(|x| x.parse().unwrap())
        .map(find_best_split)
        .sum();

    Number(ans as i64)
}
