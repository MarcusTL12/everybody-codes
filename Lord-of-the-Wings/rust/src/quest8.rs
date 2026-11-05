use ndarray::ArrayView2;

use crate::{
    Quest,
    QuestResult::{self, Unsigned},
    util::input_to_grid,
};

pub const PARTS: Quest = [part1, part2, part3];

fn count_light(
    grid: ArrayView2<u8>,
    start: [usize; 2],
    dir: [isize; 2],
) -> u64 {
    let mut shadow = 0;
    let mut hit = 0;

    let mut pos = start;

    while let Some(v) = grid.get(pos) {
        pos = [
            pos[0].wrapping_add_signed(dir[0]),
            pos[1].wrapping_add_signed(dir[1]),
        ];

        if !v.is_ascii_digit() {
            continue;
        }

        let h = v - b'0';
        if shadow < h {
            hit += 1;
        }

        shadow = shadow.max(h) - 1;
    }

    hit
}

fn part1(mut input: String) -> QuestResult {
    input.push('\n');
    let grid = input_to_grid(input.as_bytes());

    let (h, w) = grid.dim();

    let ans = (0..h)
        .map(|y| [y, 0])
        .chain((1..w).map(|x| [0, x]))
        .map(|start| count_light(grid.view(), start, [1, 1]))
        .sum();

    Unsigned(ans)
}

fn part2(mut input: String) -> QuestResult {
    input.push('\n');
    let grid = input_to_grid(input.as_bytes());

    let (h, w) = grid.dim();

    let ans1: u64 = (0..h)
        .map(|y| [y, 0])
        .chain((1..w).map(|x| [0, x]))
        .map(|start| count_light(grid.view(), start, [1, 1]))
        .sum();

    let ans2: u64 = (0..h)
        .map(|y| [y, w - 1])
        .chain((0..w - 1).map(|x| [0, x]))
        .map(|start| count_light(grid.view(), start, [1, -1]))
        .sum();

    let ans3: u64 = (0..h)
        .map(|y| [y, w - 1])
        .map(|start| count_light(grid.view(), start, [0, -1]))
        .sum();

    let ans4: u64 = (0..h)
        .map(|y| [y, w - 1])
        .chain((0..w - 1).map(|x| [h - 1, x]))
        .map(|start| count_light(grid.view(), start, [-1, -1]))
        .sum();

    let ans5: u64 = (0..h)
        .map(|y| [y, 0])
        .chain((1..w).map(|x| [h - 1, x]))
        .map(|start| count_light(grid.view(), start, [-1, 1]))
        .sum();

    let ans6: u64 = (0..h)
        .map(|y| [y, 0])
        .map(|start| count_light(grid.view(), start, [0, 1]))
        .sum();

    Unsigned(ans1 + ans2 + ans3 + ans4 + ans5 + ans6)
}

fn part3(input: String) -> QuestResult {
    todo!("\n{input}")
}
