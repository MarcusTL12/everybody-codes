use std::collections::BTreeSet;

use ndarray::{ArrayView2, ArrayViewMut2};

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

fn update_queue<P: Fn(u8) -> bool>(
    grid: ArrayView2<u8>,
    queue: &mut BTreeSet<[usize; 2]>,
    pos: [usize; 2],
    pred: P,
) {
    for pos in neighbours(pos, grid.dim()) {
        if pred(grid[pos]) {
            queue.insert(pos);
        }
    }
}

fn fill_neighbours<P: Fn(u8) -> bool, Q: Fn(u8) -> bool>(
    mut grid: ArrayViewMut2<u8>,
    queue: &mut BTreeSet<[usize; 2]>,
    pred: P,
    qpred: Q,
    count: usize,
    turn_into: u8,
) {
    while let Some(pos) = queue.pop_first() {
        if neighbours(pos, grid.dim())
            .filter(|&p| pred(grid[p]))
            .count()
            >= count
        {
            grid[pos] = turn_into;
            update_queue(grid.view(), queue, pos, &qpred);
        }
    }
}

fn part1(input: String) -> QuestResult {
    let mut input = input.into_bytes();
    input.reserve_exact(1);
    input.push(b'\n');

    let grid = input_to_grid_mut(&mut input);

    let (h, w) = grid.dim();

    let mut queue = BTreeSet::new();

    update_queue(grid.view(), &mut queue, [h / 2, w / 2], |c| c != b'#');

    fill_neighbours(grid, &mut queue, |c| c != b'0', |c| c != b'#', 5, b'#');

    let ans = input.into_iter().filter(|&c| c == b'#').count();

    Unsigned(ans as u64)
}

fn part2(input: String) -> QuestResult {
    let mut input = input.into_bytes();
    input.reserve_exact(1);
    input.push(b'\n');

    let mut grid = input_to_grid_mut(&mut input);
    let (h, w) = grid.dim();

    let mut queue = BTreeSet::new();

    update_queue(grid.view(), &mut queue, [h / 2, w / 2], |c| c != b'#');

    fill_neighbours(
        grid.view_mut(),
        &mut queue,
        |c| c != b'0',
        |c| c != b'#',
        5,
        b'#',
    );

    for y in 0..h {
        for x in 0..w {
            if grid[[y, x]] == b'#'
                && neighbours([y, x], grid.dim())
                    .filter(|&pos| matches!(grid[pos], b'#' | b'='))
                    .count()
                    != 8
            {
                grid[[y, x]] = b'=';
                queue.insert([y, x]);
                // update_queue(grid.view(), &mut queue, [y, x], |c| c == b'#');
            }
        }
    }

    let mut new_queue = BTreeSet::new();

    for pos in queue {
        update_queue(grid.view(), &mut new_queue, pos, |c| c == b'#');
    }

    let mut queue = new_queue;

    fill_neighbours(grid, &mut queue, |c| c == b'=', |c| c == b'#', 5, b'=');

    let ans = input.into_iter().filter(|&c| c == b'=').count();

    Unsigned(ans as u64)
}

fn part3(input: String) -> QuestResult {
    todo!("\n{input}")
}
