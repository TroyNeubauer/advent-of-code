use std::collections::HashSet;

use crate::traits::*;

pub struct S;

#[derive(Clone, Debug)]
struct Vertex {
    name: String,
    connections: Vec<u8>,
}

#[derive(Clone, Debug)]
struct Graph(Vec<Vertex>);

/*
const SPACES: &str = "                                                                                                ";

fn i(indent: usize) -> &'static str {
    &SPACES[..indent]
}
*/

impl Graph {
    fn parse_line(line: &str) -> (&str, &str) {
        let mut parts = line.split('-');
        let a = parts.next().unwrap();
        let b = parts.next().unwrap();
        (a, b)
    }

    fn parse(input: Input) -> Graph {
        let lines: Vec<_> = input.lines().collect();

        let mut vertex_names = HashSet::new();
        for line in &lines {
            let (a, b) = Self::parse_line(line);
            vertex_names.insert(a);
            vertex_names.insert(b);
        }
        let mut vertices = Vec::new();
        let vertex_names: Vec<_> = vertex_names.iter().copied().collect();
        for name in &vertex_names {
            vertices.push(Vertex {
                name: name.to_string(),
                connections: Vec::new(),
            });
        }
        for vert_index in 0..vertices.len() {
            let vert = unsafe { &mut *vertices.as_mut_ptr().add(vert_index) };
            for line in &lines {
                let (a, b) = Self::parse_line(line);
                if vert.name == a {
                    //There is an edge that starts with us
                    let mut b_index = None;
                    for (i, name) in vertex_names.iter().enumerate() {
                        if *name == b {
                            b_index = Some(i);
                        }
                    }
                    let b_index = b_index.unwrap();
                    vert.connections.push(b_index.try_into().unwrap());

                    let vert_b = unsafe { &mut *vertices.as_mut_ptr().add(b_index) };
                    vert_b.connections.push(vert_index.try_into().unwrap());
                }
            }
        }
        Graph(vertices)
    }

    fn start(&self) -> u8 {
        for (i, vert) in self.0.iter().enumerate() {
            if vert.name == "start" {
                return i.try_into().unwrap();
            }
        }
        unreachable!()
    }

    fn count_paths(
        &self,
        current_num: u8,
        visited: &mut HashSet<u8>,
        current_path: &mut Vec<u8>,
    ) -> usize {
        let current = &self.0[current_num as usize];
        if visited.contains(&current_num) {
            //We've been here before
            if current.name.chars().next().unwrap().is_ascii_uppercase() {
            } else if current.name != "end" {
                return 0;
            }
        }

        visited.insert(current_num);
        current_path.push(current_num);
        if current.name == "end" { 
            current_path.pop();
            return 1;
        }

        let mut sum = 0;
        for conn in &current.connections {
            sum += self.count_paths(*conn, visited, current_path);
        }

        visited.remove(&current_num);
        current_path.pop();

        sum
    }

    fn count_paths_double(
        &self,
        current_num: u8,
        visited: &mut HashMap<u8, u8>,
        current_path: &mut Vec<u8>,
        indent: usize,
    ) -> usize {
        let current = &self.0[current_num as usize];
        let visited_count = *visited.entry(current_num).or_default();

        if current.name.chars().next().unwrap().is_ascii_uppercase() {
        } else if current.name != "end" {
            if visited_count >= 2 {
                return 0;
            } else if visited_count == 1 {
                //make sure we haven't visited another small cave twice already
                let mut max: Option<(u8, u8)> = None;
                for (index, count) in visited.iter() {
                    let n = &self.0[*index as usize];
                    if !n.name.chars().next().unwrap().is_ascii_uppercase()
                        && n.name != "end"
                        && (max.is_none() || max.unwrap().1 < *count)
                    {
                        max = Some((*index, *count));
                    }
                }
                let (_max_index, max_count) = max.unwrap();

                if max_count >= 2 { 
                    return 0;
                }
            }
        }

        *visited.entry(current_num).or_default() += 1;
        current_path.push(current_num);
        if current.name == "end" { 
            current_path.pop();
            return 1;
        }

        let mut sum = 0;
        for conn in &current.connections {
            if self.0[*conn as usize].name != "start" {
                sum += self.count_paths_double(*conn, visited, current_path, indent + 2);
            }
        }

        *visited.entry(current_num).or_default() -= 1;
        current_path.pop();

        sum
    }
}

impl crate::traits::AocDay for S {
    fn part1(&self, input: Input) -> Output {
        let g = Graph::parse(input); 
        let start = g.start();

        let mut current_path = Vec::new();
        let mut visited = HashSet::new();
        g.count_paths(start, &mut visited, &mut current_path).into()
    }

    fn part2(&self, input: Input) -> Output {
        let g = Graph::parse(input);
        let start = g.start();

        let mut current_path = Vec::new();
        let mut visited = HashMap::new();
        g.count_paths_double(start, &mut visited, &mut current_path, 0)
            .into()
    }
}
