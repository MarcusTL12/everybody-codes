use std::{collections::BTreeMap, mem::swap};

use bitarray::BitArray;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    Quest,
    QuestResult::{self, Unsigned},
    util::parse_bytes_u64,
};

pub const PARTS: Quest = [part1, part2, part3];

fn parse_input<const N: usize>(input: String) -> Vec<(u64, BitArray<N>)> {
    input
        .as_bytes()
        .split(|&c| c == b'\n')
        .map(|l| {
            let mut parts = l.split(|&c| c == b' ');
            parts.next();
            let energy = parse_bytes_u64(
                parts
                    .next()
                    .unwrap()
                    .iter()
                    .skip(b"energy=".len())
                    .take_while(|c| c.is_ascii_digit()),
            );

            let mut connections = BitArray::new();

            for id in parts.next().unwrap()[b"connections=".len()..]
                .split(|&c| c == b',')
                .map(parse_bytes_u64)
            {
                assert!(id <= N as u64 * usize::BITS as u64);
                connections.set(id as usize - 1, true);
            }

            (energy, connections)
        })
        .collect()
}

fn propagate_energy<const N: usize>(
    network: &mut [(u64, BitArray<N>)],
    id: usize,
    energy: u64,
) {
    if energy == 0 {
        return;
    }

    network[id].0 += energy;

    for other_id in network[id].1.trues_iter() {
        propagate_energy(network, other_id, energy - 1);
    }
}

fn part1(input: String) -> QuestResult {
    let mut network = parse_input::<1>(input);

    let (startid, _) = network
        .iter()
        .enumerate()
        .max_by_key(|(_, (_, c))| c.count_ones())
        .unwrap();

    propagate_energy(&mut network, startid, 5);

    let ans = network.into_iter().map(|(energy, _)| energy).sum();

    Unsigned(ans)
}

fn part2(input: String) -> QuestResult {
    let mut network = parse_input::<2>(input);

    let (startid, _) = network
        .iter()
        .enumerate()
        .max_by_key(|(_, (_, c))| c.count_ones())
        .unwrap();

    propagate_energy(&mut network, startid, 10);

    let ans = network.into_iter().map(|(energy, _)| energy).sum();

    Unsigned(ans)
}

fn propagate_energy2<const N: usize>(
    network: &[(u64, BitArray<N>)],
    id: usize,
    energy: u64,
    limit: u64,
) -> u64 {
    let mut energies: Vec<_> = network.iter().map(|&(x, _)| x).collect();
    let mut exploded = BitArray::<N>::new();

    let mut queue1 = BTreeMap::new();
    let mut queue2 = BTreeMap::new();

    queue1.insert((id, energy), 1);

    while !queue1.is_empty() {
        for (&(id, energy), &n) in &queue1 {
            if exploded[id] {
                continue;
            }

            if energy > 1 {
                for other_id in network[id].1.trues_iter() {
                    if !exploded[other_id] {
                        *queue2.entry((other_id, energy - 1)).or_insert(0) += n;
                    }
                }
            }

            energies[id] += energy * n;
        }

        for (id, &energy) in energies.iter().enumerate() {
            if energy >= limit && !exploded[id] {
                exploded.set(id, true);
                for other_id in network[id].1.trues_iter() {
                    if !exploded[other_id] {
                        *queue2.entry((other_id, 10)).or_insert(0) += 1;
                    }
                }
            }
        }

        queue1.clear();
        swap(&mut queue1, &mut queue2);
    }

    energies.into_iter().filter(|&x| x < limit).sum()
}

fn part3(input: String) -> QuestResult {
    const LIMIT: u64 = 1000000;

    let network = parse_input::<2>(input);

    let ans = (0..network.len())
        .into_par_iter()
        .map(|i| propagate_energy2(&network, i, 10, LIMIT))
        .max()
        .unwrap();

    Unsigned(ans)
}
