use std::collections::BTreeSet;

use ndarray::ArrayView2;

use crate::{
    Quest,
    QuestResult::{self, Unsigned},
    util::input_to_grid_mut,
};

pub const PARTS: Quest = [part1, part2, part3];

const DIRS: [[isize; 2]; 8] = [
    [1, 0],
    [1, 1],
    [0, 1],
    [-1, 1],
    [-1, 0],
    [-1, -1],
    [0, -1],
    [1, -1],
];

fn neighbours(
    [y, x]: [usize; 2],
    (h, w): (usize, usize),
) -> impl Iterator<Item = [usize; 2]> {
    DIRS.into_iter()
        .filter_map(move |[dy, dx]| {
            y.checked_add_signed(dy).zip(x.checked_add_signed(dx))
        })
        .filter_map(move |(y, x)| (y < h && x < w).then_some([y, x]))
}

fn update_queue(
    grid: ArrayView2<u8>,
    queue: &mut BTreeSet<[usize; 2]>,
    new_land: [usize; 2],
) {
    for pos in neighbours(new_land, grid.dim()) {
        if grid[pos] != b'#' {
            queue.insert(pos);
        }
    }
}

fn part1(input: String) -> QuestResult {
    let mut input = input.into_bytes();
    input.reserve_exact(1);
    input.push(b'\n');

    let mut grid = input_to_grid_mut(&mut input);

    let (h, w) = grid.dim();

    let mut queue = BTreeSet::new();

    update_queue(grid.view(), &mut queue, [h / 2, w / 2]);

    while let Some(pos) = queue.pop_first() {
        if neighbours(pos, grid.dim())
            .filter(|&p| grid[p] != b'0')
            .count()
            >= 5
        {
            grid[pos] = b'#';
            update_queue(grid.view(), &mut queue, pos);
        }
    }

    let ans = input.into_iter().filter(|&c| c == b'#').count();

    Unsigned(ans as u64)
}

fn part2(input: String) -> QuestResult {
    todo!("\n{input}")
}

fn part3(input: String) -> QuestResult {
    todo!("\n{input}")
}
