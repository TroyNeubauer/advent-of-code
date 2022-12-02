use crate::traits::*;

pub struct S;

impl crate::traits::AocDay for S {
    fn part1(&self, input: Input) -> Output {
        let mut mat = Matrix::new_from_single_nums(input.as_str()).unwrap();
        let mut flashes = 0;
        for _ in 0..100 {
            for cell in mat.iter_mut() {
                *cell += 1;
            }
            let mut flash_pos = Vec::new();
            loop {
                let mut inner_flash_count = 0;
                for (row, col) in mat.cells() {
                    if *mat.get(row, col) > 9 {
                        if !flash_pos.contains(&(row, col)) {
                            inner_flash_count += 1;
                            flash_pos.push((row, col));
                            println!("{} {} flashing", row, col);
                            for adj in mat.neighbor_iter_mut(row, col) {
                                *adj += 1;
                            }
                        }
                    }
                }
                flashes += inner_flash_count;
                if inner_flash_count == 0 {
                    break;
                }
            }
            for (row, col) in &flash_pos {
                mat.set(*row, *col, 0);
            }
        }

        flashes.into()
    }

    fn part2(&self, input: Input) -> Output {
        let mut mat = Matrix::new_from_single_nums(input.as_str()).unwrap();
        let mut i = 0;
        loop {
            let mut flashes_this_it = 0;
            for cell in mat.iter_mut() {
                *cell += 1;
            }
            let mut flash_pos = Vec::new();
            loop {
                let mut inner_flash_count = 0;
                for (row, col) in mat.cells() {
                    if *mat.get(row, col) > 9 {
                        if !flash_pos.contains(&(row, col)) {
                            inner_flash_count += 1;
                            flash_pos.push((row, col));
                            for adj in mat.neighbor_iter_mut(row, col) {
                                *adj += 1;
                            }
                        }
                    }
                }
                flashes_this_it += inner_flash_count;
                if inner_flash_count == 0 {
                    break;
                }
            }
            for (row, col) in &flash_pos {
                mat.set(*row, *col, 0);
            }
            i += 1;
            //They 1 index i
            if flashes_this_it == mat.len() {
                break i.into();
            }
        }
    }
}
