use std::fs;

pub fn main() {
    let input = String::from_utf8(fs::read("input.txt").unwrap()).unwrap();
    /*let input = r#"forward 5
down 5
forward 8
up 3
down 8
forward 2"#;*/
    let mut horiz = 0;
    let mut aim = 0;
    let mut depth = 0;
    for command in input.lines() {
        let mut parts = command.split(' ');
        let c = parts.next().unwrap();
        let num = parts.next().unwrap().parse::<u32>().unwrap();
        match c {
            "down" => aim += num,
            "up" => aim -= num,
            "forward" => {
                horiz += num;
                depth += aim * num;
            }
            //"down" => depth += num,
            _ => panic!("{}", c),
        }
    }
    println!("{}", horiz * depth);
}
