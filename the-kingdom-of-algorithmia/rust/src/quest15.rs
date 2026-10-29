use std::collections::VecDeque;

use bitarray::BitArray;
use ndarray::Array2;

use crate::{
    Quest,
    QuestResult::{self, Number},
    util::input_to_grid_mut,
};

pub const PARTS: Quest = [part2; 3];

const DIRS: [[isize; 2]; 4] = [[1, 0], [-1, 0], [0, 1], [0, -1]];

fn part2(input: String) -> QuestResult {
    let mut bytes = input.into_bytes();
    bytes.reserve_exact(1);
    bytes.push(b'\n');
    let mut grid = input_to_grid_mut(&mut bytes);

    let sx = grid
        .row(0)
        .indexed_iter()
        .find_map(|(x, &v)| (v == b'.').then_some(x))
        .unwrap();

    grid[[0, sx]] = b'!';

    let herb_mask = grid.iter().fold(0usize, |bits, &v| match v {
        b'A'..=b'Z' => bits | 1 << (v - b'A'),
        _ => bits,
    });

    let mut herb_mapping = [0; 26];

    for (i, j) in BitArray([herb_mask]).trues_iter().enumerate() {
        herb_mapping[j] = i as u8;
    }

    let n_herbs = herb_mask.count_ones();

    const N: usize = 512;

    assert!(2u32.pow(n_herbs) <= N as u32 * usize::BITS);

    let mut queue = VecDeque::new();
    let mut seen = Array2::from_elem(grid.dim(), BitArray::<N>::new());

    queue.push_back(([0, sx], 0u16, 0));
    seen[[0, sx]] |= 1.into();

    while let Some((pos, herbs, l)) = queue.pop_front() {
        for dir in DIRS {
            let Some(newpos) =
                pos[0].checked_add_signed(dir[0]).and_then(|np0| {
                    pos[1].checked_add_signed(dir[1]).map(|np1| [np0, np1])
                })
            else {
                continue;
            };

            let Some(&v) = grid.get(newpos) else {
                continue;
            };

            match v {
                b'.' => {
                    if !seen[newpos][herbs as usize] {
                        seen[newpos].set(herbs as usize, true);
                        queue.push_back((newpos, herbs, l + 1));
                    }
                }
                b'A'..=b'Z' => {
                    let new_herb_bit = 1 << herb_mapping[(v - b'A') as usize];
                    let new_herbs = herbs | new_herb_bit;

                    if !seen[newpos][new_herbs as usize] {
                        seen[newpos].set(new_herbs as usize, true);
                        queue.push_back((newpos, new_herbs, l + 1));
                    }
                }
                b'!' => {
                    if herbs.count_ones() == n_herbs {
                        return Number(l + 1);
                    }
                }
                b'#' | b'~' => {}
                _ => panic!("{v:?}"),
            }
        }
    }

    panic!()
}
