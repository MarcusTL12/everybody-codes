use ndarray::Array2;

use crate::{util, Quest, QuestResult};

pub const PARTS: Quest = [part1, part2, part3];

fn part1(input: String) -> QuestResult {
    let mut lines = input.as_bytes().split(|&x| x == b'\n');

    let runes = lines.next().unwrap().split(|&x| x == b':').nth(1).unwrap();

    lines.next();

    let inscription = lines.next().unwrap();

    QuestResult::Number(
        runes
            .split(|&x| x == b',')
            .map(|rune| {
                inscription
                    .windows(rune.len())
                    .filter(|&x| x == rune)
                    .count() as i64
            })
            .sum(),
    )
}

fn part2(input: String) -> QuestResult {
    let (runes, inscription) = input.split_once("\n\n").unwrap();

    let runes = runes.as_bytes().split(|&x| x == b':').nth(1).unwrap();

    let inscription = inscription.as_bytes();

    let mut bitmap = vec![false; inscription.len()];

    for rune in runes.split(|&x| x == b',') {
        for i in 0..inscription.len() - rune.len() + 1 {
            let r = i..i + rune.len();
            let window = &inscription[r.clone()];
            if rune == window
                || rune.iter().rev().zip(window).all(|(a, b)| a == b)
            {
                for b in &mut bitmap[r] {
                    *b = true;
                }
            }
        }
    }

    QuestResult::Number(bitmap.into_iter().filter(|&x| x).count() as i64)
}

fn part3(mut input: String) -> QuestResult {
    if input.as_bytes().last().unwrap() != &b'\n' {
        input.push('\n');
    }

    let (runes, inscription) = input.split_once("\n\n").unwrap();

    let runes = runes.as_bytes().split(|&x| x == b':').nth(1).unwrap();

    let inscription = inscription.as_bytes();

    let matrix = util::input_to_grid(inscription);

    let &[h, w] = matrix.shape() else {
        unreachable!()
    };

    let mut bitmap = Array2::from_elem([h, w], false);

    for i in 0..h {
        for j in 0..w {
            for rune in runes.split(|&x| x == b',') {
                let r = (0..rune.len()).map(|k| (k + j) % w);
                if r.clone().zip(rune).all(|(k, &c)| matrix[[i, k]] == c)
                    || r.clone()
                        .zip(rune.iter().rev())
                        .all(|(k, &c)| matrix[[i, k]] == c)
                {
                    for k in r {
                        bitmap[[i, k]] = true;
                    }
                }

                if i + rune.len() <= h {
                    let r = (0..rune.len()).map(|k| k + i);
                    if r.clone().zip(rune).all(|(k, &c)| matrix[[k, j]] == c)
                        || r.clone()
                            .zip(rune.iter().rev())
                            .all(|(k, &c)| matrix[[k, j]] == c)
                    {
                        for k in r {
                            bitmap[[k, j]] = true;
                        }
                    }
                }
            }
        }
    }

    QuestResult::Number(bitmap.into_iter().filter(|&x| x).count() as i64)
}
