use crate::traits::*;

pub struct S;

fn next_address(count: &mut u64, mask: u64, base_address: u64) -> Option<u64> {
    if *count == 2_u64.pow(mask.count_ones()) {
        return None;
    }
    let mut unset_address = base_address & !mask;
    let mut bits_cloned = 0;
    for i in 0..64 {
        if (mask >> i) & 0b1 == 1 {
            unset_address |= (*count >> bits_cloned & 0b1) << i;
            bits_cloned += 1;
        }
    }
    *count += 1;
    Some(unset_address)
}

impl crate::traits::AocDay for S {
    fn part1(&self, input: Input) -> Output {
        let mut memory: HashMap<u64, u64> = HashMap::new();
        let mut mask_on: u64 = 0;
        let mut mask_off: u64 = 0;
        for line in input.lines() {
            let data_str = line.split(" = ").nth(1).unwrap();
            if line.starts_with("mask") {
                assert_eq!(data_str.len(), 36);
                for c in data_str.chars() {
                    mask_on <<= 1;
                    mask_off <<= 1;
                    match c {
                        'X' => {}
                        '1' => mask_on |= 1,
                        '0' => mask_off |= 1,
                        _ => panic!("unknown char in mask"),
                    }
                }
            } else if line.starts_with("mem") {
                let open = line.chars().position(|c| c == '[').unwrap();
                let close = line.chars().position(|c| c == ']').unwrap();
                let address_str = &line[open + 1..close];
                let address = address_str.parse::<u64>().unwrap();
                let mut data = data_str.parse::<u64>().unwrap();
                data &= !mask_off;
                data |= mask_on;
                data &= 0x000F_FFFF_FFFF;
                memory.insert(address, data);
            } else {
                panic!("unknown command");
            }
        }
        let mut result = 0;
        for value in memory.values() {
            result += value;
        }
        result.into()
    }

    fn part2(&self, input: Input) -> Output {
        let mut memory: HashMap<u64, u64> = HashMap::new();
        let mut mask_on: u64 = 0;
        let mut mask_floating: u64 = 0;
        for line in input.lines() {
            let data_str = line.split(" = ").nth(1).unwrap();
            if line.starts_with("mask") {
                assert_eq!(data_str.len(), 36);
                mask_on = 0;
                mask_floating = 0;
                for c in data_str.chars() {
                    mask_on <<= 1;
                    mask_floating <<= 1;
                    match c {
                        'X' => mask_floating |= 1,
                        '1' => mask_on |= 1,
                        '0' => {}
                        _ => panic!("unknown char in mask"),
                    }
                }
            } else if line.starts_with("mem") {
                let open = line.chars().position(|c| c == '[').unwrap();
                let close = line.chars().position(|c| c == ']').unwrap();
                let address_str = &line[open + 1..close];
                let mut address = address_str.parse::<u64>().unwrap();
                let data = data_str.parse::<u64>().unwrap();
                address |= mask_on;
                address &= 0x000F_FFFF_FFFF;
                let mut count = 0;
                while let Some(next_addr) = next_address(&mut count, mask_floating, address) {
                    memory.insert(next_addr, data);
                }
            } else {
                panic!("unknown command");
            }
        }
        let mut result = 0;
        for value in memory.values() {
            result += value;
        }
        result.into()
    }
}
