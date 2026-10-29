use std::cmp::Reverse;

use priority_queue::PriorityQueue;

use crate::{
    Quest,
    QuestResult::{self, Number},
    util::input_to_grid_mut,
};

pub const PARTS: Quest = [part1; 3];

fn hdist(a: u8, b: u8) -> u32 {
    let diff = a.abs_diff(b) as u32;

    diff.min(10 - diff)
}

fn part1(mut input: String) -> QuestResult {
    input.reserve_exact(1);
    input.push('\n');
    let mut grid = input_to_grid_mut(unsafe { input.as_bytes_mut() });

    let mut queue = PriorityQueue::new();

    for s in grid
        .indexed_iter()
        .filter_map(|((y, x), v)| (*v == b'S').then_some([y, x]))
    {
        queue.push(s, Reverse(0));
    }

    const DIRS: [[isize; 2]; 4] = [[1, 0], [-1, 0], [0, 1], [0, -1]];

    while let Some((pos, Reverse(t))) = queue.pop() {
        let cell = grid.get_mut(pos).unwrap();

        let cur_height = match *cell {
            b'E' => return Number(t.into()),
            b'S' => 0,
            b'0'..=b'9' => *cell - b'0',
            _ => panic!(),
        };

        *cell = b'#';

        for dir in DIRS {
            let newpos = [
                (pos[0] as isize + dir[0]) as usize,
                (pos[1] as isize + dir[1]) as usize,
            ];

            match grid.get(newpos) {
                Some(b'E') => {
                    let d = hdist(cur_height, 0);

                    queue.push_increase(newpos, Reverse(t + d + 1));
                }
                Some(b'0'..=b'9') => {
                    let d = hdist(cur_height, grid[newpos] - b'0');

                    queue.push_increase(newpos, Reverse(t + d + 1));
                }
                _ => {}
            }
        }
    }

    panic!()
}
