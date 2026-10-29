use ndarray::{ArrayView2, s};

use crate::{
    Quest,
    QuestResult::{self, Number, Text},
    util::input_to_grid,
};

pub const PARTS: Quest = [part1, part2, part3];

fn solve_string(grid: ArrayView2<u8>) -> [u8; 16] {
    let mut vertical = [0u32; 4];
    let mut horizontal = [0u32; 4];

    for (i, c1) in (2..6).enumerate() {
        for c2 in (0..2).chain(6..8) {
            let c = grid[[c1, c2]];
            horizontal[i] |= 1 << (c - b'A');

            let c = grid[[c2, c1]];
            vertical[i] |= 1 << (c - b'A');
        }
    }

    let mut ans = [0; _];

    for ((h, v), c) in horizontal
        .into_iter()
        .flat_map(|h| vertical.into_iter().map(move |v| (h, v)))
        .zip(&mut ans)
    {
        *c = (h & v).trailing_zeros() as u8;
    }

    ans
}

fn runic_word(values: [u8; 16]) -> u32 {
    values
        .into_iter()
        .enumerate()
        .map(|(i, x)| (i as u32 + 1) * (x as u32 + 1))
        .sum()
}

fn part1(mut input: String) -> QuestResult {
    input.reserve_exact(1);
    input.push('\n');

    let grid = input_to_grid(input.as_bytes());

    let values = solve_string(grid);

    dbg!(runic_word(values));

    let ans = values.into_iter().map(|c| (c + b'A') as char).collect();

    Text(ans)
}

fn part2(mut input: String) -> QuestResult {
    let linelength = input.lines().next().unwrap().len();

    input.reserve_exact(2);
    input.push('\n');
    input.push('\n');

    let block_size = (linelength + 1) * 8 + 1;
    let file_height = input.len() / block_size;
    let super_w = (linelength + 1) / 9;

    let ans: u32 =
        ArrayView2::from_shape([file_height, block_size], input.as_bytes())
            .unwrap()
            .slice_move(s![.., 0..(block_size - 1)])
            .rows()
            .into_iter()
            .flat_map(|r| {
                r.into_shape_with_order([8, super_w, 9])
                    .unwrap()
                    .slice_move(s![.., .., 0..8])
                    .permuted_axes([1, 0, 2])
                    .into_outer_iter()
            })
            .map(solve_string)
            .map(runic_word)
            .sum();

    Number(ans as i64)
}

fn part3(input: String) -> QuestResult {
    todo!("\n{input}")
}
