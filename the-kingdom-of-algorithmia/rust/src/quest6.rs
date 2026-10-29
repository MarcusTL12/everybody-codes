use std::collections::{HashMap, VecDeque};

use arrayvec::ArrayVec;

use crate::{Quest, QuestResult};

pub const PARTS: Quest = [part1, part2, part3];

fn part1(input: String) -> QuestResult {
    let tree: HashMap<_, _> = input
        .split('\n')
        .map(|l| {
            let [node, rest] = l
                .split(':')
                .collect::<ArrayVec<_, 2>>()
                .into_inner()
                .unwrap();

            (node, rest.split(','))
        })
        .collect();

    let mut traceback = HashMap::new();
    let mut queue = VecDeque::new();

    queue.push_back(("RR", 0));

    let mut start1 = None;
    let mut start2 = None;

    while let Some((node, len)) = queue.pop_front() {
        if let Some(it) = tree.get(&node) {
            for nextnode in it.clone() {
                if traceback.contains_key(nextnode) {
                    continue;
                }
                if nextnode == "@" {
                    if let Some((_, other_len)) = start1 {
                        if other_len == len {
                            start1 = start2;
                            start2 = None;
                        } else {
                            start2 = Some((node, len));
                        }
                    } else {
                        start1 = Some((node, len));
                    }
                } else {
                    queue.push_back((nextnode, len + 1));
                    traceback.insert(nextnode, node);
                }
            }
        }
    }

    let mut path = vec![b'@'];
    let mut cur_node = start1.unwrap().0;
    path.extend(cur_node.as_bytes().iter().rev());

    while let Some(next_node) = traceback.get(cur_node) {
        path.extend(next_node.as_bytes().iter().rev());

        cur_node = next_node;
    }

    QuestResult::Text(path.into_iter().rev().map(|c| c as char).collect())
}

fn part2(input: String) -> QuestResult {
    let tree: HashMap<_, _> = input
        .split('\n')
        .map(|l| {
            let [node, rest] = l
                .split(':')
                .collect::<ArrayVec<_, 2>>()
                .into_inner()
                .unwrap();

            (node, rest.split(','))
        })
        .collect();

    let mut traceback = HashMap::new();
    let mut queue = VecDeque::new();

    queue.push_back(("RR", 0));

    let mut start1 = None;
    let mut start2 = None;

    while let Some((node, len)) = queue.pop_front() {
        if let Some(it) = tree.get(&node) {
            for nextnode in it.clone() {
                if traceback.contains_key(nextnode) {
                    continue;
                }
                if nextnode == "@" {
                    if let Some((_, other_len)) = start1 {
                        if other_len == len {
                            start1 = start2;
                            start2 = None;
                        } else {
                            start2 = Some((node, len));
                        }
                    } else {
                        start1 = Some((node, len));
                    }
                } else {
                    queue.push_back((nextnode, len + 1));
                    traceback.insert(nextnode, node);
                }
            }
        }
    }

    let mut path = vec![b'@'];
    let mut cur_node = start1.unwrap().0;
    path.push(cur_node.as_bytes()[0]);

    while let Some(next_node) = traceback.get(cur_node) {
        path.push(next_node.as_bytes()[0]);

        cur_node = next_node;
    }

    QuestResult::Text(path.into_iter().rev().map(|c| c as char).collect())
}

fn part3(input: String) -> QuestResult {
    let tree: HashMap<_, _> = input
        .split('\n')
        .map(|l| {
            let [node, rest] = l
                .split(':')
                .collect::<ArrayVec<_, 2>>()
                .into_inner()
                .unwrap();

            (node, rest.split(','))
        })
        .collect();

    let mut traceback = HashMap::new();
    let mut queue = VecDeque::new();

    queue.push_back(("RR", 0));

    let mut start1 = None;
    let mut start2 = None;

    while let Some((node, len)) = queue.pop_front() {
        if let Some(it) = tree.get(&node) {
            for nextnode in it.clone() {
                if traceback.contains_key(nextnode) {
                    continue;
                }
                if nextnode == "@" {
                    if let Some((_, other_len)) = start1 {
                        if other_len == len {
                            start1 = start2;
                            start2 = None;
                        } else {
                            start2 = Some((node, len));
                        }
                    } else {
                        start1 = Some((node, len));
                    }
                } else if nextnode != "BUG" && nextnode != "ANT" {
                    queue.push_back((nextnode, len + 1));
                    traceback.insert(nextnode, node);
                }
            }
        }
    }

    let mut path = vec![b'@'];
    let mut cur_node = start1.unwrap().0;
    path.push(cur_node.as_bytes()[0]);

    while let Some(next_node) = traceback.get(cur_node) {
        path.push(next_node.as_bytes()[0]);

        cur_node = next_node;
    }

    QuestResult::Text(path.into_iter().rev().map(|c| c as char).collect())
}
