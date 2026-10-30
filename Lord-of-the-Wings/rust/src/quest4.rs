use std::cmp::{Ordering, Reverse};

use arrayvec::ArrayVec;

use crate::{
    Quest,
    QuestResult::{self, Unsigned},
};

pub const PARTS: Quest = [part1, part2, part3];

fn parsebytes<I: Iterator<Item = u8>>(bytes: I) -> u32 {
    bytes.fold(0, |n, d| 10 * n + (d - b'0') as u32)
}

fn parseinput(input: &str) -> impl Iterator<Item = (u32, u32)> {
    input.as_bytes().split(|&c| c == b'\n').map(|l| {
        let mut it = l.split(|&c| c == b' ').step_by(3);
        let id = parsebytes(
            it.next()
                .unwrap()
                .split_once(|&c| c == b'=')
                .unwrap()
                .1
                .iter()
                .cloned(),
        );

        let t = parsebytes(
            it.next()
                .unwrap()
                .split_once(|&c| c == b'=')
                .unwrap()
                .1
                .iter()
                .cloned(),
        );

        (id, t)
    })
}

fn part1(input: String) -> QuestResult {
    let mut record = Vec::new();

    for (id, t) in parseinput(&input) {
        if record.len() < id as usize {
            record.resize(id as usize, 0);
        }

        record[id as usize - 1] += t;
    }

    record.sort();

    let ans: u32 = record
        .into_iter()
        .enumerate()
        .map(|(i, t)| (i as u32 + 1) * t)
        .sum();

    Unsigned(ans.into())
}

fn part2(input: String) -> QuestResult {
    let mut record = Vec::new();

    for (id, t) in parseinput(&input) {
        if record.len() < id as usize {
            record.resize(id as usize, (Reverse(0u32), 0));
        }

        record[id as usize - 1].0.0 += 1;
        record[id as usize - 1].1 += t;
    }

    record.sort();

    let ans: u32 = record
        .into_iter()
        .enumerate()
        .map(|(i, (_, t))| (i as u32 + 1) * t)
        .sum();

    Unsigned(ans.into())
}

fn parseinputfull(input: &str) -> impl Iterator<Item = [u32; 4]> {
    input.as_bytes().split(|&c| c == b'\n').map(|l| {
        let mut vals = [0; 4];

        for (s, d) in l
            .split(|&c| c == b' ')
            .map(|b| {
                parsebytes(
                    b.split_once(|&c| c == b'=').unwrap().1.iter().cloned(),
                )
            })
            .zip(&mut vals)
        {
            *d = s;
        }

        vals
    })
}

fn assign_points<T, O: Ord, S: FnMut(&T) -> O, F: Fn(&T) -> bool>(
    points: &mut [u32],
    data: &mut [(u32, T)],
    mut score: S,
    failed: F,
    max_points: u32,
) {
    data.sort_by_key(|(_, x)| score(x));

    let mut last_award = 0;

    for (i, award) in (0..=max_points).rev().enumerate().take(data.len()) {
        if failed(&data[i].1) {
            break;
        }

        if let Some(Ordering::Equal) = i.checked_sub(1).and_then(|j| {
            data.get(j).map(|(_, x)| score(x).cmp(&score(&data[i].1)))
        }) {
            points[data[i].0 as usize] += last_award;
        } else {
            points[data[i].0 as usize] += award;
            last_award = award;
        }
    }
}

fn part3(input: String) -> QuestResult {
    let mut record = Vec::new();

    let mut num_ids = 0;

    for [id, q, p, t] in parseinputfull(&input) {
        num_ids = num_ids.max(id);

        if record.len() < q as usize {
            record.resize(q as usize, Vec::new());
        }

        let q = &mut record[q as usize - 1];

        for new_id in q.len() as u32..id {
            q.push((new_id, ArrayVec::<_, 3>::new()));
        }

        let id = &mut q[id as usize - 1].1;

        while id.len() < p as usize {
            id.push(id.last().cloned().unwrap_or(0));
        }

        for i in p as usize - 1..id.len() {
            id[i] += t;
        }
    }

    let mut points = vec![0; num_ids as usize];

    for mut quest in record {
        for (p, award) in (0..3).zip([50, 100, 150]) {
            assign_points(
                &mut points,
                &mut quest,
                |x| x.get(p).cloned().unwrap_or(u32::MAX),
                |x| x.get(p).is_none(),
                award,
            );
        }
    }

    points.sort_by_key(|&x| Reverse(x));

    let (_, _, ans) = points.into_iter().enumerate().fold(
        (u32::MAX, 0, 0),
        |(last_score, last_place, checksum), (i, score)| {
            if score == last_score {
                (last_score, last_place, checksum + last_place * score)
            } else {
                let place = i as u32 + 1;
                (score, place, checksum + place * score)
            }
        },
    );

    Unsigned(ans.into())
}
