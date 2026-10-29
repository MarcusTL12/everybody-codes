use std::{cmp::Reverse, collections::BTreeMap};

use ndarray::ArrayView2;
use rayon::{iter::ParallelIterator, str::ParallelString};

use crate::{
    Quest,
    QuestResult::{self, Number},
    util::input_to_grid_mut,
};

pub const PARTS: Quest = [part1, part1, part3];

fn shoot(
    grid: ArrayView2<u8>,
    y: usize,
    x: usize,
    power: usize,
) -> Option<[usize; 2]> {
    let trj = (0..power)
        .scan([y, x], |[y, x], _| {
            *x += 1;
            *y = y.wrapping_sub(1);

            Some([*y, *x])
        })
        .chain((0..power).scan(
            [y.wrapping_sub(power), x + power],
            |[y, x], _| {
                *x += 1;

                Some([*y, *x])
            },
        ));

    let mut x = x + 2 * power;
    let mut y = y.wrapping_sub(power);

    let trj = trj.chain((0..).map_while(|_| {
        x += 1;
        y = y.wrapping_add(1);

        (y != grid.dim().0).then_some([y, x])
    }));

    for [y, x] in trj {
        if let (Some(&b'T' | &b'H'), Some(&b'.') | None) =
            (grid.get([y, x]), grid.get([y.wrapping_sub(1), x]))
        {
            return Some([y, x]);
        }
    }

    None
}

fn part1(mut input: String) -> QuestResult {
    input.reserve_exact(1);
    input.push('\n');
    let mut grid = input_to_grid_mut(unsafe { input.as_bytes_mut() });

    let mut towers = BTreeMap::new();

    for r in grid.rows().into_iter() {
        for (j, &x) in r.into_iter().enumerate() {
            if let b'T' | b'H' = x {
                if let Some(h) = towers.get_mut(&j) {
                    *h += 1;
                } else {
                    towers.insert(j, 1);
                }
            }
        }
    }

    let (h, _) = grid.dim();

    let base = [h - 2, 1];

    let mut ans = 0;

    while !towers.is_empty() {
        let (hit, power, height) = (1..)
            .flat_map(|power| (0..3).map(move |height| (power, height)))
            .find_map(|(power, height)| {
                shoot(grid.view(), base[0] - height, base[1], power)
                    .map(|hit| (hit, power, height))
            })
            .unwrap();

        let Some(spot) = grid.get_mut(hit) else {
            panic!();
        };

        match *spot {
            b'H' => ans += 2 * power * (height + 1),
            b'T' => ans += power * (height + 1),
            _ => panic!(),
        }

        *spot = b'.';

        let Some(v) = towers.get_mut(&hit[1]) else {
            panic!();
        };

        *v -= 1;

        if *v == 0 {
            towers.remove(&hit[1]);
        }
    }

    Number(ans as i64)
}

fn shoot_meteor(height: u32, power: u32, mx: u32, my: u32) -> Option<[u32; 2]> {
    let stage1 = (0..=power).map(|i| [i, i + height]);
    let stage2 = (1..power).map(|i| [power + i, power + height]);
    let stage3 =
        (0..=(power + height)).map(|i| [2 * power + i, power + height - i]);

    let trj_proj = stage1.chain(stage2).chain(stage3);
    let trj_met = (0..=mx).rev().zip((0..=my).rev()).map(|(x, y)| [x, y]);

    trj_proj
        .zip(trj_met)
        .find_map(|(p, m)| (p == m).then_some(p))
}

fn find_best_shot(mx: u32, my: u32) -> Option<u32> {
    let mut best_height = None;
    let mut best_score = None;

    for delay in 0..mx.min(my) {
        let [mx, my] = [mx - delay, my - delay];

        if best_height.is_some_and(|h| my + 2 <= 2 * h) {
            break;
        }

        if let Some((h, Reverse(score))) = (0..=my.div_ceil(2))
            .flat_map(|power| (0..3).map(move |height| (height, power)))
            .filter_map(move |(height, power)| {
                shoot_meteor(height, power, mx, my)
                    .map(|[_, y]| (y, Reverse((height + 1) * power)))
            })
            .max()
            && best_height.is_none_or(|h2| h > h2)
        {
            best_height = Some(h);
            best_score = Some(score);
        }
    }

    println!("{:?} -> {:?}", [mx, my], best_score);

    best_score
}

fn part3(input: String) -> QuestResult {
    let ans: u32 = input
        .par_lines()
        .map(|l| {
            let (x, y) = l.split_once(' ').unwrap();

            let x = x.parse().unwrap();
            let y = y.parse().unwrap();

            find_best_shot(x, y).unwrap()
        })
        .sum();

    Number(ans.into())
}
