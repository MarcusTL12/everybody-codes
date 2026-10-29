use std::{
    collections::{BTreeSet, HashSet, VecDeque},
    iter::repeat_n,
};

use num_traits::Zero;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    Quest,
    QuestResult::{self, Number},
};

pub const PARTS: Quest = [part1, part2, part3];

fn part1(input: String) -> QuestResult {
    let (_, ans) = input
        .split(',')
        .filter_map(|r| match &r[0..1] {
            "U" => Some(r[1..].parse::<i64>().unwrap()),
            "D" => Some(-r[1..].parse::<i64>().unwrap()),
            _ => None,
        })
        .fold((0, 0), |(y, maxy), dy| (y + dy, maxy.max(y + dy)));

    Number(ans)
}

fn part2(input: String) -> QuestResult {
    let tiles: HashSet<_> = input
        .lines()
        .flat_map(|l| {
            l.split(',')
                .flat_map(|r| {
                    repeat_n(
                        match &r[0..1] {
                            "R" => [1, 0, 0],
                            "L" => [-1, 0, 0],
                            "U" => [0, 1, 0],
                            "D" => [0, -1, 0],
                            "F" => [0, 0, 1],
                            "B" => [0, 0, -1],
                            _ => panic!(),
                        },
                        r[1..].parse().unwrap(),
                    )
                })
                .scan([0; 3], |pos, dp| {
                    for (q, dq) in pos.iter_mut().zip(&dp) {
                        *q += dq;
                    }

                    Some(*pos)
                })
        })
        .collect();

    Number(tiles.len() as i64)
}

fn murkiness(
    segment: &HashSet<[i32; 3]>,
    leaves: &HashSet<[i32; 3]>,
    y: i32,
) -> u32 {
    let mut ans = 0;

    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();

    queue.push_back(([0, y, 0], 0));
    seen.insert([0, y, 0]);

    const DIRS: [[i32; 3]; 6] = [
        [1, 0, 0],
        [-1, 0, 0],
        [0, 1, 0],
        [0, -1, 0],
        [0, 0, 1],
        [0, 0, -1],
    ];

    while let Some((pos, l)) = queue.pop_front() {
        if leaves.contains(&pos) {
            ans += l;
        }

        for dir in DIRS {
            let mut newpos = pos;

            for (q, dq) in newpos.iter_mut().zip(&dir) {
                *q += dq;
            }

            if !seen.contains(&newpos) && segment.contains(&newpos) {
                seen.insert(newpos);
                queue.push_back((newpos, l + 1));
            }
        }
    }

    ans
}

fn part3(input: String) -> QuestResult {
    let mut trunk = BTreeSet::new();
    let mut segments = HashSet::new();

    let leaves: HashSet<[i32; 3]> = input
        .lines()
        .map(|l| {
            l.split(',')
                .flat_map(|r| {
                    repeat_n(
                        match &r[0..1] {
                            "R" => [1, 0, 0],
                            "L" => [-1, 0, 0],
                            "U" => [0, 1, 0],
                            "D" => [0, -1, 0],
                            "F" => [0, 0, 1],
                            "B" => [0, 0, -1],
                            _ => panic!(),
                        },
                        r[1..].parse().unwrap(),
                    )
                })
                .fold([0; 3], |mut pos, dp| {
                    for (q, dq) in pos.iter_mut().zip(&dp) {
                        *q += dq;
                    }

                    segments.insert(pos);
                    if pos[0].is_zero() && pos[2].is_zero() {
                        trunk.insert(pos[1]);
                    }

                    pos
                })
        })
        .collect();

    let ans = trunk
        .into_par_iter()
        .map(|ty| murkiness(&segments, &leaves, ty))
        .min()
        .unwrap();

    Number(ans.into())
}
