use num_integer::lcm;

use crate::{
    Quest,
    QuestResult::{self, Number, Text},
};

pub const PARTS: Quest = [part1, part2, part3];

fn parse_input(input: &str) -> (Vec<u32>, Vec<Vec<&str>>) {
    let mut lines = input.lines();

    let steps: Vec<u32> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    lines.next().unwrap();

    let mut wheels = vec![Vec::new(); steps.len()];

    for mut l in lines {
        for wheel in wheels.iter_mut() {
            let Some((cat, rest)) = l.split_at_checked(3) else {
                break;
            };

            if let Some((_, r)) = rest.split_at_checked(1) {
                l = r;
            } else {
                l = "";
            }

            if cat != "   " {
                wheel.push(cat);
            }
        }
    }

    (steps, wheels)
}

fn part1(input: String) -> QuestResult {
    let (steps, wheels) = parse_input(&input);

    let mut positions = vec![0; steps.len()];

    for _ in 0..100 {
        for ((pos, step), wheel) in
            positions.iter_mut().zip(&steps).zip(&wheels)
        {
            let l = wheel.len() as u32;
            *pos = (*pos + step) % l;
        }
    }

    Text(
        wheels
            .into_iter()
            .zip(positions)
            .map(|(wheel, i)| wheel[i as usize])
            .intersperse(" ")
            .collect(),
    )
}

fn count_coins(positions: &[u32], wheels: &[Vec<&str>]) -> u32 {
    let mut counts = [0u8; 256];

    for &c in positions
        .iter()
        .zip(wheels)
        .flat_map(|(&i, w)| w[i as usize].as_bytes().iter().step_by(2))
    {
        counts[c as usize] += 1;
    }

    counts.iter().map(|c| c.saturating_sub(2) as u32).sum()
}

fn accumulate_coins(steps: &[u32], wheels: &[Vec<&str>], n: u32) -> u32 {
    let mut positions = vec![0; steps.len()];

    let mut coins = 0;

    for _ in 0..n {
        for ((pos, step), wheel) in positions.iter_mut().zip(steps).zip(wheels)
        {
            let l = wheel.len() as u32;
            *pos = (*pos + step) % l;
        }

        coins += count_coins(&positions, wheels);
    }

    coins
}

fn part2(input: String) -> QuestResult {
    let (steps, wheels) = parse_input(&input);

    let period = wheels.iter().map(|w| w.len() as u32).reduce(lcm).unwrap();

    const TARGET: u64 = 202420242024;

    dbg!(period);
    let coins_per_period = dbg!(accumulate_coins(&steps, &wheels, period));

    let whole_periods = TARGET / period as u64;
    let steps_left = TARGET % period as u64;

    let coins_from_whole_periods = whole_periods * coins_per_period as u64;

    let ans = coins_from_whole_periods
        + accumulate_coins(&steps, &wheels, steps_left as u32) as u64;

    Number(ans.try_into().unwrap())
}

fn part3(input: String) -> QuestResult {
    todo!("\n{input}")
}
