use crate::traits::*;

pub struct S;

mod part1 {
    fn from_base_10_str(input: &str) -> Result<i32, std::num::ParseIntError> {
        std::str::FromStr::from_str(input)
    }

    fn int32(inupt: &str) -> nom::IResult<&str, i32> {
        nom::combinator::map_res(nom::character::complete::digit1, from_base_10_str)(inupt)
    }

    pub fn get_line(input: &str) -> nom::IResult<&str, bool> {
        let (input, (min, _, max, _, c, _, password, _)) = nom::sequence::tuple((
            int32,
            nom::bytes::complete::tag("-"),
            int32,
            nom::bytes::complete::tag(" "),
            nom::character::complete::anychar,
            nom::bytes::complete::tag(": "),
            nom::character::complete::not_line_ending,
            nom::character::complete::line_ending,
        ))(input)?;
        let mut char_count = 0;
        for pass_char in password.chars() {
            if pass_char == c {
                char_count += 1;
            }
        }
        Ok((input, char_count >= min && char_count <= max))
    }
}

mod part2 {

    fn from_base_10_str(input: &str) -> Result<i32, std::num::ParseIntError> {
        std::str::FromStr::from_str(input)
    }

    fn int32(inupt: &str) -> nom::IResult<&str, i32> {
        nom::combinator::map_res(nom::character::complete::digit1, from_base_10_str)(inupt)
    }

    pub fn get_line(input: &str) -> nom::IResult<&str, bool> {
        let (input, (a_index, _, b_index, _, c, _, password, _)) = nom::sequence::tuple((
            int32,
            nom::bytes::complete::tag("-"),
            int32,
            nom::bytes::complete::tag(" "),
            nom::character::complete::anychar,
            nom::bytes::complete::tag(": "),
            nom::character::complete::not_line_ending,
            nom::character::complete::line_ending,
        ))(input)?;
        let mut char_count = 0;
        if password
            .chars()
            .nth(usize::try_from(a_index - 1).expect("Cannot convert to negitive usize!"))
            .expect("invalid index")
            == c
        {
            char_count += 1;
        }
        if password
            .chars()
            .nth(usize::try_from(b_index - 1).expect("Cannot convert to negitive usize!"))
            .expect("invalid index")
            == c
        {
            char_count += 1;
        }
 
        Ok((input, char_count == 1))
    }
}

impl crate::traits::AocDay for S {
    fn part1(&self, input: Input) -> Output {
        let mut input = input.as_str();
        let mut count = 0;
        while input.len() > 0 {
            let result = part1::get_line(input).expect("Failed to parse input");
            if result.1 {
                count += 1;
            }
            input = result.0;
        }
        count.into()
    }

    fn part2(&self, input: Input) -> Output {
        let mut input = input.as_str();
        let mut count = 0;
        while input.len() > 0 {
            let result = part2::get_line(input).expect("Failed to parse input");
            if result.1 {
                count += 1;
            }
            input = result.0;
        }
        count.into()
    }
}
