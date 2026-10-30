use rayon::{iter::ParallelIterator, slice::ParallelSlice};

use crate::{
    Quest,
    QuestResult::{self, Unsigned},
};

pub const PARTS: Quest = [part1, part2, part3];

fn decode_iso(l: &[u8; 13]) -> u16 {
    (0..4)
        .map(|i| {
            ((l[4 + i] - b'0') as u16).pow(3 - i as u32)
                * (l[9 + i] - b'0') as u16
        })
        .sum()
}

fn part1(input: String) -> QuestResult {
    let ans: u16 = input
        .as_bytes()
        .split(|&c| c == b'\n')
        .map(|l| decode_iso(l.try_into().unwrap()))
        .sum();

    Unsigned(ans.into())
}

fn iso2_kernel<I1: Iterator<Item = u8>, I2: Iterator<Item = u8>>(
    iso_iter: I1,
    digits_iter: I2,
) -> u64 {
    digits_iter
        .zip(iso_iter)
        .enumerate()
        .map(|(i, (x, b))| (b as u64).pow(i as u32) * x as u64)
        .sum()
}

fn decode_iso2(l: &[u8]) -> u64 {
    let i_space = l
        .iter()
        .enumerate()
        .find_map(|(i, &c)| (c == b' ').then_some(i))
        .unwrap();

    let iso_code = &l[4..i_space];
    let number = &l[i_space + 1..];

    let iso_iter = iso_code.iter().map(|&c| c - b'0').rev().cycle();
    let digits_iter = number.iter().map(|&c| c - b'0').rev();

    iso2_kernel(iso_iter, digits_iter)
}

fn part2(input: String) -> QuestResult {
    let ans: u64 = input
        .as_bytes()
        .split(|&c| c == b'\n')
        .map(decode_iso2)
        .sum();

    Unsigned(ans)
}

fn make_digit_iterator(n: u64) -> impl Iterator<Item = u8> {
    (0..).scan(n, |n, _| {
        (*n != 0).then(|| {
            let d = *n % 10;
            *n /= 10;
            d as u8
        })
    })
}

fn veryfy_iso2(l: &[u8]) -> Option<u64> {
    let i_space = l
        .iter()
        .enumerate()
        .find_map(|(i, &c)| (c == b' ').then_some(i))
        .unwrap();

    let iso_code = &l[5..i_space];
    let number = l[i_space + 1..]
        .iter()
        .fold(0, |n, d| 10 * n + (d - b'0') as u64);

    let iso_iter = iso_code.iter().map(|&c| c - b'0').rev().cycle();
    let digits_iter = make_digit_iterator(number);

    let ans = iso2_kernel(iso_iter.clone(), digits_iter);

    if (1..number)
        .any(|n| iso2_kernel(iso_iter.clone(), make_digit_iterator(n)) == ans)
    {
        return None;
    }

    Some(ans)
}

fn part3(input: String) -> QuestResult {
    let ans: u64 = input
        .as_bytes()
        .par_split(|&c| c == b'\n')
        .filter_map(veryfy_iso2)
        .sum();

    Unsigned(ans)
}
