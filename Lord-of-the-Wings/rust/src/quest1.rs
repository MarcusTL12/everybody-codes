use std::simd::{Select, Simd, StdFloat, cmp::SimdPartialOrd, num::SimdFloat};

use crate::{
    Quest,
    QuestResult::{self, Unsigned},
};

pub const PARTS: Quest = [part1, part2, part3];

fn part1(input: String) -> QuestResult {
    let ans: u64 = input
        .split(',')
        .map(|x| x.parse().unwrap())
        .map(|x: u64| ((x as f64).sqrt().ceil() as u64).pow(2) - x)
        .sum();

    Unsigned(ans)
}

fn part2(input: String) -> QuestResult {
    let ans: u64 = input
        .split(',')
        .map(|x| x.parse().unwrap())
        .map(|x: u64| if x <= 8 { 8 - x } else { 4 * x.div_ceil(4) - x })
        .sum();

    Unsigned(ans)
}

const N: usize = 16;

fn p3_chunk(xs: Simd<f32, N>) -> Simd<f32, N> {
    const ONE: Simd<f32, N> = Simd::splat(1.0);
    const EIGHT: Simd<f32, N> = Simd::splat(8.0);

    let mask = xs.simd_le(EIGHT);

    let ns = (((xs + ONE).sqrt() - ONE) * Simd::splat(0.5)).ceil();

    let ys_tmp = ns + ns + ONE;
    let ys = ys_tmp.mul_add(ys_tmp, -ONE);

    let ys = mask.select(EIGHT, ys);

    ys - xs
}

fn part3(input: String) -> QuestResult {
    let mut chunks = input
        .split(',')
        .map(|x| x.parse::<u32>().unwrap() as f32)
        .array_chunks();

    let acc: Simd<_, _> = chunks.by_ref().map(|xs| p3_chunk(xs.into())).sum();

    let mut r = chunks.into_remainder();
    let r = [0; _].map(|_| r.next().unwrap_or(8.0));

    let acc = acc + p3_chunk(r.into());

    let ans = acc.reduce_sum() as u64;

    Unsigned(ans)
}
