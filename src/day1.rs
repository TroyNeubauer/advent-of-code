use std::{collections::HashMap, fs};

fn main2() {
    let input = String::from_utf8(fs::read("input.txt").unwrap()).unwrap();
    //let input = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    let input: Vec<_> = input
        .lines()
        .filter_map(|line| line.parse::<u32>().ok())
        .collect();

    let mut count = 0;
    for i in 0..input.len() - 1 {
        if input[i + 1] > input[i] {
            count += 1;
        }
    }

    println!("{}", count);
}

fn sum(data: &[u32], index: usize) -> u32 {
    data.iter().skip(index).take(3).sum()
}

fn main() {

    let input = String::from_utf8(fs::read("input.txt").unwrap()).unwrap();
    //let input = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    let input: Vec<_> = input
        .lines()
        .filter_map(|line| line.parse::<u32>().ok())
        .collect();

    let mut count = 0;
    for i in 0..input.len() {
        let first = sum(&input, i);
        let second = sum(&input, i + 1);
        if second > first {
            count += 1;
        }
    }
    println!("Count {}", count);
}
