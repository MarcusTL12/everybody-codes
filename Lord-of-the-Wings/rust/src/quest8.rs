use ndarray::{ArrayView2, ArrayViewMut2};

use crate::{
    Quest,
    QuestResult::{self, Unsigned},
    util::{input_to_grid, input_to_grid_mut},
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

fn startpos_iter(
    (h, w): (usize, usize),
    i: usize,
) -> Box<dyn Iterator<Item = [usize; 2]>> {
    match i {
        0 => Box::new((0..h).map(|y| [y, 0]).chain((1..w).map(|x| [0, x]))),
        1 => Box::new(
            (0..h)
                .map(move |y| [y, w - 1])
                .chain((0..w - 1).map(|x| [0, x])),
        ),
        2 => Box::new((0..h).map(move |y| [y, w - 1])),
        3 => Box::new(
            (0..h)
                .map(move |y| [y, w - 1])
                .chain((0..w - 1).map(move |x| [h - 1, x])),
        ),
        4 => Box::new(
            (0..h)
                .map(|y| [y, 0])
                .chain((1..w).map(move |x| [h - 1, x])),
        ),
        5 => Box::new((0..h).map(|y| [y, 0])),
        _ => panic!(),
    }
}

const DIRS: [[isize; 2]; 6] =
    [[1, 1], [1, -1], [0, -1], [-1, -1], [-1, 1], [0, 1]];

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

    let ans: u64 = (0..6)
        .flat_map(|i| {
            startpos_iter(grid.dim(), i)
                .map(move |start| count_light(grid.view(), start, DIRS[i]))
        })
        .sum();

    Unsigned(ans)
}

fn grow_step(
    mut grid: ArrayViewMut2<u8>,
    start: [usize; 2],
    dir: [isize; 2],
) -> (u64, u64) {
    let mut shadow = 0;
    let mut matured = 0;
    let mut hit = 0;

    let mut pos = start;

    while let Some(v) = grid.get_mut(pos) {
        if let b'X'..=b'Z' = *v
            && shadow > 0
        {
            shadow -= 1;
        }

        match v {
            b'X' => *v = b'Y',
            b'Y' => *v = b'Z',
            b'Z' => *v = b'1',
            b'0'..=b'9' => {
                let h = *v - b'0';
                if shadow < h {
                    hit += 1;
                    if h == 9 {
                        *v = b'X';
                        matured += 1;
                    } else {
                        *v += 1;
                    }
                }

                shadow = shadow.max(h) - 1;
            }
            _ => {}
        }

        pos = [
            pos[0].wrapping_add_signed(dir[0]),
            pos[1].wrapping_add_signed(dir[1]),
        ];
    }

    (matured, hit)
}

fn part3(input: String) -> QuestResult {
    let mut input = input.into_bytes();
    input.push(b'\n');
    let mut grid = input_to_grid_mut(&mut input);

    let mut total_warriors = 0;
    let mut total_energy = 0;

    let mut i = 0;

    while total_warriors < 100000 {
        let (mature, energy) = startpos_iter(grid.dim(), i)
            .map(|start| grow_step(grid.view_mut(), start, DIRS[i]))
            .fold((0, 0), |(matured, energy), (new_mature, new_energy)| {
                (matured + new_mature, energy + new_energy)
            });

        i = match i {
            0..=4 => i + 1,
            5 => 0,
            _ => unreachable!(),
        };

        total_warriors += mature;
        total_energy += energy;
    }

    Unsigned(total_energy)
}
