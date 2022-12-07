use slab_tree::*;
use std::{path::PathBuf, str::Lines};

use anyhow::Result;
use util::{runner_main, AocDay, Input, Output};

struct Day1;

#[derive(Debug)]
enum Entry {
    Dir(String),
    File { name: String, size: usize },
}

fn parse(i: &str) -> Tree<Entry> {
    let mut tree = TreeBuilder::new().with_root(Entry::Dir("/".into())).build();
    let mut current: NodeId = tree.root_id().unwrap();
    for line in i.lines().skip(1) {
        let c = line.chars().nth(0).unwrap();
        if c == '$' {
            if line.starts_with("$ cd") {
                let mut path = String::new();
                scanf::sscanf!(line, "$ cd {}", path).unwrap();
                if path == ".." {
                    current = tree.get(current).unwrap().parent().unwrap().node_id();
                } else {
                    let mut c = tree.get_mut(current).unwrap();
                    let child = c.append(Entry::Dir(path));
                    current = child.node_id();
                }
            } else if line.starts_with("$ ls") {
            } else {
                panic!("{}", line);
            }
        } else if c.is_digit(10) {
            // file entry
            let mut s = line.split(" ");
            let size: usize = s.next().unwrap().parse().unwrap();
            let name = s.next().unwrap();
            let mut c = tree.get_mut(current).unwrap();
            c.append(Entry::File {
                size,
                name: name.to_owned(),
            });
        } else if c == 'd' {
            let mut name = String::new();
            scanf::sscanf!(line, "dir {}", name).unwrap();
        }
    }
    let mut s = String::new();
    tree
}

fn size_1(node: NodeId, tree: &Tree<Entry>, count: &mut usize) -> usize {
    let this = tree.get(node).unwrap();
    match this.data() {
        Entry::File { name: _, size } => *size,
        Entry::Dir(_) => {
            let mut s = 0;
            for child in this.children() {
                s += size_1(child.node_id(), tree, count);
            }
            if s < 100000 {
                *count += s;
            }

            s
        }
    }
}

fn size_2(
    node: NodeId,
    tree: &Tree<Entry>,
    size_needed: usize,
    canidates: &mut Vec<usize>,
) -> usize {
    let this = tree.get(node).unwrap();
    match this.data() {
        Entry::File { name: _, size } => *size,
        Entry::Dir(_) => {
            let mut s = 0;
            for child in this.children() {
                s += size_2(child.node_id(), tree, size_needed, canidates);
            }
            if s >= size_needed {
                canidates.push(s);
            }

            s
        }
    }
}

impl AocDay for Day1 {
    fn part1(&self, i: Input) -> Output {
        let s = i.as_str();
        let tree = parse(s);

        let mut c = 0;
        size_1(tree.root_id().unwrap(), &tree, &mut c);
        c.into()
    }

    fn part2(&self, i: Input) -> Output {
        let mut v = Vec::new();

        let s = i.as_str();
        let tree = parse(s);
        let mut c = 0;

        let total_space = size_1(tree.root_id().unwrap(), &tree, &mut c);
        let remaining = 70000000 - total_space;
        let size_needed = 30000000 - remaining;

        size_2(tree.root_id().unwrap(), &tree, size_needed, &mut v);
        v.iter().min().unwrap().into()
    }
}

fn main() {
    let d = Day1;
    runner_main(&d, 2022, 7);
}
