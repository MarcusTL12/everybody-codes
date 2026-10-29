use crate::{util, Quest, QuestResult};

pub const PARTS: Quest = [part1, part1, part3];

fn solve(mut input: String, dirs: &[[i32; 2]]) -> QuestResult {
    if input.as_bytes().last().unwrap() != &b'\n' {
        input.push('\n');
    }

    let data = unsafe { input.as_bytes_mut() };

    let mut matrix = util::input_to_grid_mut(data);

    let &[h, w] = matrix.shape() else {
        unreachable!()
    };

    for x in matrix.iter_mut() {
        match *x {
            b'.' => *x = 0,
            b'#' => *x = 1,
            _ => (),
        }
    }

    let mut done = false;

    while !done {
        done = true;

        for i in 1..h - 1 {
            for j in 1..w - 1 {
                if matrix[[i, j]] != 0
                    && dirs
                        .iter()
                        .map(|[di, dj]| {
                            [(i as i32 + di) as usize, (j as i32 + dj) as usize]
                        })
                        .all(|k| matrix[k] >= matrix[[i, j]])
                {
                    matrix[[i, j]] += 1;
                    done = false;
                }
            }
        }
    }

    QuestResult::Number(matrix.into_iter().map(|x| *x as i64).sum())
}

fn part1(input: String) -> QuestResult {
    solve(input, &[[1, 0], [-1, 0], [0, 1], [0, -1]])
}

fn part3(input: String) -> QuestResult {
    solve(
        input,
        &[
            [1, 0],
            [1, 1],
            [0, 1],
            [-1, 1],
            [-1, 0],
            [-1, -1],
            [0, -1],
            [1, -1],
        ],
    )
}
