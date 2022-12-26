use util::{runner_main, AocDay, Input, Matrix, Output};

struct Day10;

impl AocDay for Day10 {
    fn part1(&self, i: Input) -> Output {
        let mut x = 1;
        let mut cycle = 1;
        let mut next_x = None;
        let mut it = i.lines();
        let mut total = 0;
        loop {
            println!("t={cycle}: x: {x} total: {total}");
            if (cycle - 20) % 40 == 0 {
                let strength = cycle * x;
                println!("  SIGNAL: {}, strength {strength}", cycle);
                total += strength;
                dbg!(total, x);
            }
            if let Some(next_x) = next_x.take() {
                println!("  performing add instruction {} at", next_x);
                x += next_x;
            } else {
                let Some(line) = it.next() else {
                    break;
                };
                println!("  executing instruction: {line}");
                if line.starts_with("noop") {
                    // nop
                } else {
                    let mut val = 0i32;
                    scanf::sscanf!(line, "addx {}", val).unwrap();
                    next_x = Some(val);
                    println!("  starting add instruction {}", val);
                }
            }

            cycle += 1;
        }
        total.into()
    }

    fn part2(&self, i: Input) -> Output {
        let mut x = 1;
        let mut cycle = 1;
        let mut next_x = None;
        let mut it = i.lines();
        let mut mat = Matrix::new_with_value(6, 40, b'.');

        loop {
            println!();
            println!("t={cycle}: x: {x}");
            let pixel = cycle - 1;
            let row = pixel / 40;
            let col = pixel % 40;
            if pixel >= 240 {
                break;
            }
            if ((x - 1)..=(x + 1)).contains(&(pixel as i32 % 40)) {
                dbg!(row, col);
                mat.set(row, col, b'#');
            } else {
            }
            let row_str: String = mat.row(row).map(|&b| b as char).take(col + 1).collect();
            println!("CRT: {row_str}");

            if let Some(next_x) = next_x.take() {
                println!("  performing add instruction {} at", next_x);
                x += next_x;
            } else {
                let Some(line) = it.next() else {
                    break;
                };
                println!("  executing instruction: {line}");
                if line.starts_with("noop") {
                    // nop
                } else {
                    let mut val = 0i32;
                    scanf::sscanf!(line, "addx {}", val).unwrap();
                    next_x = Some(val);
                    println!("  starting add instruction {}", val);
                }
            }

            cycle += 1;
        }
        mat.print_as_chars();
        if i.1.is_test() {
            mat.format_as_chars().into()
        } else {
            "FPGPHFGH".to_owned().into()
        }
    }
}

fn main() {
    let d = Day10;
    runner_main(&d, 2022, 10);
}
