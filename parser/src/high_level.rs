use crate::low_level::{Low, Query};
use anyhow::{bail, Result};
use enum_map::{Enum, EnumMap};
use select::node::Node;
use smallvec::SmallVec;

pub struct AocPage {
    low: Low,
}

#[derive(Copy, Clone, Debug, Enum)]
pub enum ProblemStage {
    /// Part1 is unsolved, part2 is locked (0 stars)
    Part1,
    /// Part1 is solved, part2 is unsolved (1 stars)
    Part2,
    /// Both parts complete (2 stars)
    Complete,
}

impl AocPage {
    pub fn new(html: &str) -> Result<Self> {
        Ok(Self {
            low: Low::new(html)?,
        })
    }

    /// Returns the current stage of the problem
    /// Returns Err if the current stage cannot be deduced
    pub fn get_problem_stage(&self) -> Result<ProblemStage> {
        // We have many different ways of sniffing out what part of the problem we are on.
        // Including:
        // 1. The number of day_success elements
        // 2. The number of parts already solved
        // 3. How many part descriptions there are

        let success: SmallVec<[Node; 2]> = self.low.day_success().collect();
        let answers: SmallVec<[String; 2]> = self.low.puzzel_answers().collect();
        let p1 = self.low.p1_node();
        let p2 = self.low.p2_node();

        let success_stage = if success.is_empty() {
            Some(ProblemStage::Part1)
        } else {
            // could be either part2 or complete so we dont vote
            None
        };

        let answers_stage = Some(match answers.len() {
            0 => ProblemStage::Part1,
            1 => ProblemStage::Part2,
            _ => ProblemStage::Complete,
        });

        let p1_stage = match (p1, p2) {
            (None, None) => bail!("part 1 and part 2 nodes missing from page"),
            (None, Some(_)) => bail!("page contains part 2 node, but part 1 node missing"),
            (Some(_), None) => Some(ProblemStage::Part1),
            (Some(_), Some(_)) => None, // Could be either part 2 or complete
        };
        let mut votes: EnumMap<_, u8> = enum_map::enum_map! {
            ProblemStage::Part1 => 0,
            ProblemStage::Part2 => 0,
            ProblemStage::Complete => 0,
        };

        for vote in [success_stage, answers_stage, p1_stage].into_iter() {
            if let Some(vote) = vote {
                votes[vote] += 1;
            }
        }
        let winners: SmallVec<[(ProblemStage, u8); 2]> =
            votes.into_iter().filter(|(_, votes)| *votes != 0).collect();

        match winners.len() {
            0 => bail!("no votes cast for problem stage"),
            1 => Ok(winners[0].0),
            _ => bail!("mutiple winners: {:?}", winners),
        }
    }

    pub fn test_cases(&self) -> Result<TestCases> {
        let stage = self.get_problem_stage()?;
        // we assume the first code block in part 1 is the input for both test cases
        let mut blocks = self.low.code_blocks(Query::Part1);
        let input = blocks.next().map(|node| node.text());

        // we assume the last code block is the answer for the test case
        let part1_out = self
            .low
            .test_case_answer_blocks(Query::Part1)
            .last()
            .map(|node| node.text());

        let part2_out = self
            .low
            .test_case_answer_blocks(Query::Part2)
            .last()
            .map(|node| node.text());

        Ok(match stage {
            ProblemStage::Part1 => TestCases::Part1 {
                part1: TestCase {
                    input,
                    output: part1_out,
                },
            },
            ProblemStage::Part2 | ProblemStage::Complete => TestCases::Part2 {
                part1: TestCase {
                    input: input.clone(),
                    output: part1_out,
                },
                part2: TestCase {
                    input,
                    output: part2_out,
                },
            },
        })
    }

    pub fn embedded_puzzel_input(&self) -> Option<String> {
        self.low.embedded_puzzel_input()
    }
}

/// Best effort test case data scraped from the page
#[derive(Debug, PartialEq, Eq)]
pub enum TestCases {
    Part1 { part1: TestCase },
    Part2 { part1: TestCase, part2: TestCase },
}

#[derive(Debug, PartialEq, Eq)]
pub struct TestCase {
    pub input: Option<String>,
    pub output: Option<String>,
}

impl TestCases {
    pub fn has_none(&self) -> bool {
        match &self {
            TestCases::Part1 { part1 } => part1.has_none(),
            TestCases::Part2 { part1, part2 } => part1.has_none() && part2.has_none(),
        }
    }

    pub fn has_all(&self) -> bool {
        match &self {
            TestCases::Part1 { part1: _ } => false,
            TestCases::Part2 { part1, part2 } => part1.has_all() && part2.has_all(),
        }
    }

    pub fn has_all_part1(&self) -> bool {
        self.part1().has_all()
    }

    pub fn has_all_part2(&self) -> bool {
        self.part2().map(|p| p.has_all()).unwrap_or(false)
    }

    pub fn part1(&self) -> &TestCase {
        match &self {
            TestCases::Part1 { part1 } => &part1,
            TestCases::Part2 { part1, part2: _ } => &part1,
        }
    }

    pub fn part2(&self) -> Option<&TestCase> {
        match &self {
            TestCases::Part1 { part1: _ } => None,
            TestCases::Part2 { part1: _, part2 } => Some(&part2),
        }
    }
}

impl TestCase {
    pub fn has_none(&self) -> bool {
        self.input.is_none() && self.input.is_none()
    }

    pub fn has_all(&self) -> bool {
        self.input.is_some() && self.input.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    /// Asserts that the aoc page has all test case information and it matches
    #[track_caller]
    fn assert_test_cases(aoc: &AocPage, input: &str, p1_out: &str, p2_out: &str) {
        assert_eq!(
            aoc.test_cases().unwrap(),
            TestCases::Part2 {
                part1: TestCase {
                    input: Some(input.to_owned()),
                    output: Some(p1_out.to_owned()),
                },
                part2: TestCase {
                    input: Some(input.to_owned()),
                    output: Some(p2_out.to_owned()),
                },
            }
        );
    }

    #[test_log::test]
    fn day10_2015_part1() {
        let p = AocPage::new(include_str!("../test_files/part1/2015/day10.html")).unwrap();
        //assert_test_cases(&p, "", "", "");
        assert!(p.test_cases().unwrap().has_none());
    }

    #[test_log::test]
    fn day11_2018_part1() {
        let p = AocPage::new(include_str!("../test_files/part1/2018/day11.html")).unwrap();
        assert_eq!(p.embedded_puzzel_input().unwrap(), "3031");

        // TODO

        // this one is complicated and doesn't contain any easy test cases, but we still want
        // `answer_blocks` to return all the `<code><em>` blocks so that we don't have seperate
        // logic from the easy case
    }

    #[test_log::test]
    fn day1_2019_part2() {
        let p = AocPage::new(include_str!("../test_files/part2/2019/day1.html")).unwrap();
        assert!(p.embedded_puzzel_input().is_none());

        assert!(p.test_cases().unwrap().has_none());
        //assert_eq!(l.puzzel_answers().collect::<Vec<_>>(), &["3412531"]);
    }

    #[test_log::test]
    fn day2_2021_complete() {
        let p = AocPage::new(include_str!("../test_files/complete/2021/day2.html")).unwrap();

        assert_test_cases(
            &p,
            r#"
forward 5
down 5
forward 8
up 3
down 8
forward 2
"#
            .trim_start(),
            "150",
            "900",
        );
    }

    #[test_log::test]
    fn day1_2022_complete() {
        let p = AocPage::new(include_str!("../test_files/complete/2022/day1.html")).unwrap();

        assert_test_cases(
            &p,
            r#"
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"#
            .trim_start(),
            "24000",
            "45000",
        );
    }

    #[test_log::test]
    fn day2_2022_complete() {
        let start = std::time::Instant::now();

        let p = AocPage::new(include_str!("../test_files/complete/2022/day2.html")).unwrap();

        assert_test_cases(
            &p,
            r#"
A Y
B X
C Z
"#
            .trim_start(),
            "15",
            "12",
        );

        println!("took {:?}", start.elapsed());
    }

    #[test_log::test]
    fn day3_2022_complete() {
        let start = std::time::Instant::now();

        let p = AocPage::new(include_str!("../test_files/complete/2022/day3.html")).unwrap();

        assert_test_cases(
            &p,
            r#"
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"#
            .trim_start(),
            "157",
            "70",
        );
    }
    /*

        #[test_log::test]
        fn day4_2022_complete() {
            let start = std::time::Instant::now();

            let l = Low::new(include_str!("../../test_files/complete/2022/day4.html")).unwrap();
            assert!(l.p1_node().is_some());
            assert!(l.p2_node().is_some());
            assert!(l.embedded_puzzel_input().is_none());

            assert_found_nodes(
                &[
                    r#"
    2-4,6-8
    2-3,4-5
    5-7,7-9
    2-8,3-7
    6-6,4-6
    2-6,4-8
    "#
                    .trim_start(),
                    r#"
    .234.....  2-4
    .....678.  6-8

    .23......  2-3
    ...45....  4-5

    ....567..  5-7
    ......789  7-9

    .2345678.  2-8
    ..34567..  3-7

    .....6...  6-6
    ...456...  4-6

    .23456...  2-6
    ...45678.  4-8
    "#
                    .trim_start(),
                ],
                l.code_blocks(Query::Both),
            );

            assert_test_case_answers(&l, "2", "4");

            println!("took {:?}", start.elapsed());
        }
        */
}
