use std::collections::HashMap;

use crate::traits::*;

pub struct S;

fn get_name<'a>(color: u32, names: &'a HashMap<&str, u32>) -> Option<&'a str> {
    names
        .iter()
        .find_map(|(key, &val)| if color == val { Some(*key) } else { None })
}

fn contains_deep(
    color: u32,
    target_color: u32,
    graph: &[Vec<(u32, u32)>],
    backtrack: &mut HashMap<u32, u32>,
    names: &HashMap<&str, u32>,
    indent: u32,
    target_count: u32,
) -> bool {
    if target_color == color {
        return false;
    }

    for (suspect, _count) in &graph[color as usize] {
        if *suspect == target_color {
            return true;
        }
        if contains_deep(
            *suspect,
            target_color,
            graph,
            backtrack,
            names,
            indent + 1,
            target_count,
        ) {
            return true;
        }
    }
    false
}

#[allow(clippy::type_complexity)]
fn parse(input: &str) -> (Vec<Vec<(u32, u32)>>, HashMap<&str, u32>) {
    let lines: Vec<&str> = input.lines().collect();
    let mut graph: Vec<Vec<(u32, u32)>> = vec![Vec::new(); lines.len()];
    let mut names: HashMap<&str, u32> = HashMap::new();

    for line in &lines {
        let color = line.split(" bags").next().unwrap();
        names.insert(color, names.len() as u32);
    }

    for line in &lines {
        let mut it = line.split(" bags contain ");
        let color_str = it.next().unwrap();
        let rest = it.next().unwrap();
        let color = names.get(color_str).unwrap();
        for child_color_str in rest.split(", ") {
            if child_color_str == "no other bags." {
                continue;
            }
            let mut sub = 4;
            if child_color_str.ends_with('.') {
                sub = 5;
            }
            let number_pos = child_color_str.chars().position(|c| c == ' ').unwrap();
            let count = &child_color_str[..number_pos].parse::<u32>().unwrap();
            let child_color_str_better =
                &child_color_str[number_pos + 1..child_color_str.len() - sub].trim();
            let child_color = names.get(child_color_str_better).unwrap();
            graph[*color as usize].push((*child_color, *count));
        }
    }
    (graph, names)
}

fn sum_deep(
    color: u32,
    graph: &[Vec<(u32, u32)>],
    names: &HashMap<&str, u32>,
    indent: u32,
) -> usize {
    let mut result: usize = 0;
    for (suspect, count) in &graph[color as usize] {
        result +=
            sum_deep(*suspect, graph, names, indent + 1) * (*count as usize) + *count as usize;
    }
    result
}

impl crate::traits::AocDay for S {
    fn part1(&self, input: Input) -> Output {
        let mut counter = 0;
        let (graph, names) = parse(input.as_str());

        let target_color = *names.get("shiny gold").unwrap() as u32;
        let mut backtrack: HashMap<u32, u32> = HashMap::new();
        for connection in 0..graph.len() {
            if contains_deep(
                connection as u32,
                target_color as u32,
                &graph,
                &mut backtrack,
                &names,
                0,
                1,
            ) {
                counter += 1;
            }
        }
        counter.into()
    }

    fn part2(&self, input: Input) -> Output {
        let (graph, names) = parse(input.as_str());

        let target_color = *names.get("shiny gold").unwrap() as u32;
        let count = sum_deep(target_color, &graph, &names, 0);
        count.into()
    }
}
