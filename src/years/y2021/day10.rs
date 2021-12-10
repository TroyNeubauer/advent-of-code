use crate::traits::*;

pub struct S;

impl crate::traits::AocDay for S {
    fn part1(&self, input: Input) -> Output {
        let mut count = 0;
        let open = |c| {
            if c == '(' || c == '[' || c == '{' || c == '<' {
                true
            } else {
                false
            }
        };

        let close = |c| {
            if c == ')' || c == ']' || c == '}' || c == '>' {
                true
            } else {
                false
            }
        };
        'line: for line in input.lines() {
            let mut map: HashMap<char, u32> = HashMap::new();
            //let mut wrong_char_type = None;
            for c in line.chars() {
                if open(c) {
                    let a = map.entry(c).or_insert_with(|| 0);
                    *a += 1;
                    println!("{} - {}", c, *a);
                } else if close(c) {
                    let err = match map.get_mut(&c) {
                        Some(v) => {
                            if *v == 0 {
                                true
                            } else {
                                *v -= 1;
                                false
                            }
                        }
                        None => true,
                    };
                    if err {
                        let value = match c {
                            ')' => 3,
                            ']' => 57,
                            '}' => 1197,
                            '>' => 25137,
                            _ => unreachable!(),
                        };
                        println!("Found illegal: {} - {}", value, c);
                        count += value;
                        continue 'line;
                    }
                }
            }
        }
        count.into()
    }

    fn part2(&self, input: Input) -> Output {
        todo!()
    }
}
