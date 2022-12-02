use crate::traits::*;

pub struct S;

impl crate::traits::AocDay for S {
    fn part1(&self, input: Input) -> Output {
        let terrain: Vec<_> = input.lines().collect();

        let mut row = 0;
        let mut col = 0;
        let mut count = 0;

        while row < terrain.len() {
            let line = terrain[row].as_bytes();
            if *line.get(col % line.len()).unwrap() == '#' as u8 {
                count += 1;
            }

            row += 1;
            col += 3;
        }

        count.into()
    }

    fn part2(&self, input: Input) -> Output {
        let terrain: Vec<_> = input.lines().collect();
        let params = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
        let mut total = -1i64;
        for param in params {
            let mut row = 0;

            let mut col = 0;
            let mut count = 0;

            while row < terrain.len() {
                let line = terrain[row].as_bytes();
                if *line.get(col % line.len()).unwrap() == '#' as u8 {
                    count += 1;
                }

                col += param.0;
                row += param.1;
            }
            if total == -1 {
                total = count;
            } else {
                total *= count;
            }
        }
        total.into()
    }
}
