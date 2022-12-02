use crate::traits::*;

pub struct S;

fn next(map: &HashMap<u32, u32>, last: u32, turn: u32) -> u32 {
    let next = match map.get(&last) {
        Some(last_turn) => {
            if *last_turn == turn - 1 {
                0
            } else {
                turn - 1 - last_turn
            }
        }
        None => 0,
    };
    next
}

impl crate::traits::AocDay for S {
    fn part1(&self, input: Input) -> Output {
        let end_turn = 2020;

        let mut turn = 1;
        let mut map: HashMap<u32, u32> = HashMap::new();
        let mut last = 0;
        for num in input.nums_comma_separated() {
            map.insert(num, turn);
            turn += 1;
            last = num;
        }
        while turn <= end_turn {
            let next_value = next(&map, last, turn);
            map.insert(last, turn - 1);
            last = next_value;
            turn += 1;
        }

        last.into()
    }

    fn part2(&self, input: Input) -> Output {
        let end_turn = 30000000;

        let mut turn = 1;
        let mut map: HashMap<u32, u32> = HashMap::new();
        let mut last = 0;
        for num in input.nums_comma_separated() {
            map.insert(num, turn);
            turn += 1;
            last = num;
        }
        while turn <= end_turn {
            let next_value = next(&map, last, turn);
            map.insert(last, turn - 1);
            last = next_value;
            turn += 1;
        }

        last.into()
    }
}
