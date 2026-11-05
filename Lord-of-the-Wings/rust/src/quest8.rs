use crate::{
    Quest,
    QuestResult::{self, Unsigned},
    util::input_to_grid,
};

pub const PARTS: Quest = [part1, part2, part3];

fn part1(mut input: String) -> QuestResult {
    input.push('\n');
    let grid = input_to_grid(input.as_bytes());

    let (h, w) = grid.dim();

    let ans = (0..h)
        .map(|y| [y, 0])
        .chain((1..w).map(|x| [0, x]))
        .map(|[y0, x0]| {
            let mut shadow = 0;
            let mut hit = 0;

            for h in (0..)
                .map(move |n| grid.get([y0 + n, x0 + n]).cloned())
                .take_while(|v| v.is_some())
                .filter_map(|v| {
                    if let Some(b'0'..=b'9') = v {
                        v.map(|v| v - b'0')
                    } else {
                        None
                    }
                })
            {
                if shadow < h {
                    hit += 1;
                }

                shadow = shadow.max(h) - 1;
            }

            hit
        })
        .sum();

    Unsigned(ans)
}

fn part2(input: String) -> QuestResult {
    todo!("\n{input}")
}

fn part3(input: String) -> QuestResult {
    todo!("\n{input}")
}
