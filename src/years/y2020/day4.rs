use crate::traits::*;

pub struct S;

impl crate::traits::AocDay for S {
    fn part1(&self, input: Input) -> Output {
        let mut count = 0;
        for passport in input.as_str().split("\n\n") {
            if passport.contains("byr:")
                && passport.contains("iyr:")
                && passport.contains("eyr:")
                && passport.contains("hgt:")
                && passport.contains("hcl:")
                && passport.contains("ecl:")
                && passport.contains("pid:")
            {
                count += 1;
            }
        }
        count.into()
    }

    fn part2(&self, input: Input) -> Output {
        let mut counter = 0;
        for passport in input.as_str().split("\n\n") {
            let mut valid_fields = 0;
            for raw_pair in passport.split(|c| c == ' ' || c == '\n') {
                let pair = raw_pair.trim();
                let mut it = pair.split(":");
                let key = match it.next() {
                    Some(value) => value.trim(),
                    None => break,
                };
                let value = match it.next() {
                    Some(value) => value.trim(),
                    None => break,
                };
                let valid = match key {
                    "byr" => {
                        let year = value.parse::<i32>().unwrap();
                        year >= 1920 && year <= 2002
                    }
                    "iyr" => {
                        let year = value.parse::<i32>().unwrap();
                        year >= 2010 && year <= 2020
                    }
                    "eyr" => {
                        let year = value.parse::<i32>().unwrap();
                        year >= 2020 && year <= 2030
                    }
                    "hgt" => {
                        if value.len() <= 2 {
                            false
                        } else {
                            let height = value[..value.len() - 2 as usize]
                                .parse::<i32>()
                                .unwrap();
                            let unit = &value[value.len() - 2 as usize..];
                            if unit == "in" {
                                height >= 59 && height <= 76
                            } else if unit == "cm" {
                                height >= 150 && height <= 193
                            } else {
                                false
                            }
                        }
                    }
                    "hcl" => {
                        value.chars().nth(0).unwrap() == '#'
                            && value.chars().skip(1).map(|c| c.is_digit(16)).count() == 6
                    }
                    "ecl" => {
                        value == "amb"
                            || value == "blu"
                            || value == "brn"
                            || value == "gry"
                            || value == "grn"
                            || value == "hzl"
                            || value == "oth"
                    }
                    "pid" => value.chars().map(|c| c.is_digit(10)).count() == 9,
                    "cid" => false,

                    _ => panic!("invalid case"),
                };
                if valid {
                    valid_fields += 1;
                }
            }
            if valid_fields == 7 {
                counter += 1;
            }
        }
        counter.into()
    }
}
