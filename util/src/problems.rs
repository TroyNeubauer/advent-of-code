use anyhow::Result;
use chrono::Datelike;
use log::*;
use select::document::Document;
use select::node::Node;
use select::predicate::{Attr, Name, Predicate};
use serde::{Deserialize, Serialize};
use std::time::Duration as StdDuration;

use std::collections::HashMap;
use std::sync::Arc;

use crate::Output;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Test {
    #[serde(rename = "in")]
    pub input: String,
    #[serde(rename = "out")]
    pub expected_output: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Data {
    pub input: String,
    //#[serde(flatten, with = "prefix_p1")]
    pub p1: Option<Test>,
    pub p1_ans: Option<Output>,
    //#[serde(flatten, with = "prefix_p2")]
    pub p2: Option<Test>,
    pub p2_ans: Option<Output>,
    pub part: Part,
}

//with_prefix!(prefix_p1 "p1_");
//with_prefix!(prefix_p2 "p2_");

impl Data {
    pub fn is_part1_solved(&self) -> bool {
        self.p1_ans.is_some()
    }

    pub fn is_part2_solved(&self) -> bool {
        self.p2_ans.is_some()
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct Day {
    pub year: u32,
    pub day: u32,
}

/// The current part we are solving, or complete if we have both stars
#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Part {
    Part1,
    Part2,
    Complete,
}

impl std::fmt::Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match *self {
            Part::Part1 => "part1",
            Part::Part2 => "part2",
            Part::Complete => "complete",
        };
        f.write_str(s)
    }
}

impl std::fmt::Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} day {}", self.year, self.day))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Problems {
    /// Mapping of years to days to problem data
    //TODO: Make better once serde_fs supports more types as keys
    days: HashMap<u32, Data>,
    pub session: String,
}

fn parse_tests(desc_body: &str, day: Day) -> Result<(Option<Test>, Option<Test>, Part)> {
    let document = Document::from(desc_body);

    let mut test_inputs = Vec::new();
    for node in document.find(Name("pre").descendant(Name("code"))) {
        test_inputs.push(node);
    }

    let test_outputs: Vec<Node> = document
        .find(Name("em"))
        .filter_map(|s| s.text().parse::<u32>().ok().map(|_| s))
        .collect();

    let part2: Vec<_> = document.find(Name("h2").and(Attr("id", "part2"))).collect();
    if part2.len() > 1 {
        error!("Found multiple elements with the part2 tag");
        error!("Elements: {:?}", part2);
        error!("Exiting!");
        std::process::exit(1);
    }
    let part2 = part2.get(0);
    dbg!(part2);

    if test_inputs.is_empty() {
        error!("Unable to find examples for tests. Please fill in the files in `.problems`");
        error!("Possible Solutions: {:?}", test_outputs);
    }

    let mut part1_inputs = Vec::new();
    let mut part1_outputs = Vec::new();

    let mut part2_inputs = Vec::new();
    let mut part2_outputs = Vec::new();

    for input in test_inputs {
        match part2 {
            //If we know where part 2 starts in the html, and we are after it, then this is a
            //part 2 test
            Some(part) if input.index() > part.index() => part2_inputs.push(input),
            _ => part1_inputs.push(input),
        }
    }

    for output in test_outputs {
        match part2 {
            //If we know where part 2 starts in the html, and we are after it, then this is a
            //part 2 test
            Some(part) if output.index() > part.index() => part2_outputs.push(output),
            _ => part1_outputs.push(output),
        }
    }

    let zip_tests = |mut inputs: Vec<Node>,
                     mut outputs: Vec<Node>,
                     copy_from_inputs: Option<&Vec<Node>>,
                     part| {
        if inputs.is_empty() && outputs.is_empty() {
            warn!(
                "Failed to detect any test input or output for {} {}",
                day, part
            );
            None
        } else if !inputs.is_empty() && outputs.is_empty() {
            warn!("Failed to detect test output for {} {}", day, part);
            None
        } else if inputs.is_empty() && !outputs.is_empty() {
            if copy_from_inputs.is_some() && !copy_from_inputs.unwrap().is_empty() {
                //We are missing input, however we are allowed to steal from `copy_from_inputs`
                info!("Getting part 2 test input from part 1 for {}", day);
                let copy = copy_from_inputs.unwrap();
                let input = copy[copy.len() - 1];

                let output = outputs.remove(outputs.len() - 1);
                assert!(input.index() < output.index());

                Some(Test {
                    input: input.text(),
                    expected_output: output.text(),
                })
            } else {
                warn!("Failed to detect test input for {} {}", day, part);
                None
            }
        } else {
            //Sometimes we have to override which piece of output we get the input from
            const INPUT_INDEX_OVERRIDE_DAYS: [Day; 3] = [
                Day {
                    year: 2020,
                    day: 10,
                },
                Day {
                    year: 2020,
                    day: 16,
                },
                Day { year: 2021, day: 8 },
            ];
            const INPUT_INDEX_OVERRIDE_PART: [Part; 3] = [Part::Part1, Part::Part1, Part::Part1];
            const INPUT_INDEX_OVERRIDE_VALUES: [usize; 3] = [1, 1, 2];

            let mut input_index = 0;
            for (i, candidate) in INPUT_INDEX_OVERRIDE_DAYS.iter().enumerate() {
                if *candidate == day && INPUT_INDEX_OVERRIDE_PART[i] == part {
                    input_index = INPUT_INDEX_OVERRIDE_VALUES[i];
                    break;
                }
            }

            let input = inputs.remove(input_index);
            let output = outputs.remove(outputs.len() - 1);
            if input.index() >= output.index() && false {
                panic!(
                    "Test input after test output! {} >= {}",
                    input.index(),
                    output.index()
                );
            }

            Some(Test {
                input: input.text(),
                expected_output: output.text(),
            })
        }
    };

    let mut test2 = zip_tests(
        part2_inputs,
        part2_outputs,
        Some(&part1_inputs),
        Part::Part2,
    );
    let test1 = zip_tests(part1_inputs.clone(), part1_outputs, None, Part::Part1);

    // For some challenges we want to re-use the part 1 test input, even though we get valid part 2
    // input. For example 2020 day 1 part 2 has example input with markup that doesn't parse.
    // However we cant re-use all part 1 test input for part 2, because some challenges have a
    // different part 2 input on purpose
    const USE_PART1_INPUT_OVERRIDE: [Day; 5] = [
        Day { year: 2021, day: 1 },
        Day {
            year: 2020,
            day: 11,
        },
        Day { year: 2021, day: 5 },
        Day {
            year: 2021,
            day: 11,
        },
        Day {
            year: 2021,
            day: 14,
        },
    ];
    if USE_PART1_INPUT_OVERRIDE.contains(&day) {
        if let Some(p1) = &test1 {
            if let Some(p2) = &mut test2 {
                info!("Re-using part 1 test input for part 2");
                p2.input = p1.input.clone();
            }
        }
    }
    Ok((
        test1,
        test2,
        part2.map(|_| Part::Part2).unwrap_or(Part::Part1),
    ))
}

fn wait_for_time(day: Day) {
    let now_millis = chrono::Local::now().naive_local().timestamp_millis();
    // AOC releases at midnight in the eastern american timezone
    let est = chrono_tz::Tz::America__New_York;
    // We need to wait for the challenge to start
    let publish_millis = chrono::NaiveDate::from_ymd_opt(day.year as i32, 12, day.day)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_local_timezone(est)
        .unwrap()
        .timestamp_millis();

    // may be negitive if the challenge opened in the past
    let sleep_time = (publish_millis - now_millis)
        .try_into()
        .ok()
        .map(|millis| StdDuration::from_millis(millis));

    info!("Challenge publishing in {:?}", sleep_time);
    if let Some(sleep_for) = sleep_time {
        std::thread::sleep(sleep_for);
        info!("Awoke for challenge");
    }
}

const DB_PATH: &str = "./.problems";

impl Problems {
    pub fn save(&self) -> Result<()> {
        let _ = std::fs::remove_dir_all("./.problems");
        serde_fs::to_fs(&self, "./.problems")?;
        Ok(())
    }

    fn get(&self, day: Day) -> Option<&Data> {
        self.days.get(&day.day)
    }

    pub fn store(&mut self, day: Day, data: Data) {
        self.days.insert(day.day, data);
    }

    pub fn lookup(&mut self, day: Day) -> Result<Data> {
        if let Some(data) = self.get(day) {
            let new_data_not_needed = match data.part {
                Part::Part1 => data.p1_ans.is_none(),
                Part::Part2 => data.p2_ans.is_none(),
                Part::Complete => true, // we have the entire web page with answers
            };
            if new_data_not_needed && false {
                // We have the test cases we need for the current part, no need for a request
                return Ok(data.clone());
            }
        }
        loop {
            wait_for_time(day);
            let client = build_client(self.session.as_str())?;

            let description_url = format!("https://adventofcode.com/{}/day/{}", day.year, day.day);
            let desc_body = client.get(description_url).send()?.text()?;

            if desc_body.contains("the link will be enabled on the calendar the instant this puzzle becomes available") {
                //Try again in a little bit
                std::thread::sleep(std::time::Duration::from_millis(500));
                continue;
            }

            let (part1, part2, part) = parse_tests(&desc_body, day)?;
            dbg!(part);

            let data = self.days.entry(day.day).or_insert_with(|| {
                info!("Downloading input for {}", day);
                let input_url = format!(
                    "https://adventofcode.com/{}/day/{}/input",
                    day.year, day.day
                );
                let input = client.get(input_url).send().unwrap().text().unwrap();
                Data {
                    p1: part1,
                    //This only runs the first time when part2_tests is None so thats why the clone
                    //is here
                    p2: part2.clone(),
                    input,
                    p1_ans: None,
                    p2_ans: None,
                    part,
                }
            });
            data.p2 = part2;

            let a = data.clone();
            let _ = self.save();
            break Ok(a);
        }
    }

    pub fn load() -> anyhow::Result<Self> {
        Ok(serde_fs::from_fs(DB_PATH)?)
    }

    pub fn nuke(session: String) -> Result<Self> {
        let _ = std::fs::remove_dir_all(DB_PATH);
        let ret = Self {
            days: HashMap::new(),
            session,
        };
        ret.save()?;
        Ok(ret)
    }
}
