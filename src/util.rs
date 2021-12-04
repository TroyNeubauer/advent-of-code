use chrono::Datelike;
use log::*;
use select::document::Document;
use select::node::Node;
use select::predicate::{Attr, Name, Predicate};
use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::sync::Arc;

use crate::traits::Error;

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
    #[serde(rename = "p1")]
    pub part1: Option<Test>,
    #[serde(rename = "p2")]
    pub part2: Option<Test>,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct Day {
    pub year: u32,
    pub day: u32,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Part {
    Part1,
    Part2,
}

impl std::fmt::Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let s = match *self {
            Part::Part1 => "part1",
            Part::Part2 => "part2",
        };
        f.write_str(s)
    }
}

impl std::fmt::Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_fmt(format_args!("{} day {}", self.year, self.day))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Problems {
    /// Mapping of years to days to problem data
    //TODO: Make better once serde_fs supports more types as keys
    years: HashMap<u32, HashMap<u32, Data>>,
    pub session: String,
}

fn parse_tests(
    desc_body: &str,
    day: Day,
    tests_required: bool,
) -> Result<(Option<Test>, Option<Test>), Error> {
    let document = Document::from(desc_body);

    let mut test_inputs = Vec::new();
    for node in document.find(Name("pre").descendant(Name("code"))) {
        let children: Vec<_> = node.children().collect();
        if children.len() == 1 && children[0].children().next().is_none() {
            test_inputs.push(node);
        }
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

    if test_inputs.len() == 0 {
        error!("Unable to find examples for tests. Please fill in the files in `input-cache`");
        error!("Possible Solutions: {:?}", test_outputs);
        if tests_required {
            return Err("Failed parse any tests".into());
        }
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
                let input = copy[copy.len() - 1].clone();

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
            const INPUT_INDEX_OVERRIDE_DAYS: [Day; 1] = [Day { year: 2020, day: 8 }];
            const INPUT_INDEX_OVERRIDE_PART: [Part; 1] = [Part::Part1];
            const INPUT_INDEX_OVERRIDE_VALUES: [usize; 1] = [2];

            let mut input_index = 1;
            for (i, candidate) in INPUT_INDEX_OVERRIDE_DAYS.iter().enumerate() {
                if *candidate == day && INPUT_INDEX_OVERRIDE_PART[i] == part {
                    input_index = INPUT_INDEX_OVERRIDE_VALUES[i];
                    break;
                }
            }

            let input = inputs.remove(inputs.len() - input_index);
            let output = outputs.remove(outputs.len() - 1);
            assert!(input.index() < output.index());

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
    const USE_PART1_INPUT_OVERRIDE: [Day; 2] = [Day { year: 2021, day: 1 }, Day { year: 2021, day: 10 }];
    if USE_PART1_INPUT_OVERRIDE.contains(&day) {
        if let Some(p1) = &test1 {
            if let Some(p2) = &mut test2 {
                info!("Re-using part 1 test input for part 2");
                p2.input = p1.input.clone();
            }
        }
    }
    Ok((test1, test2))
}

fn wait_for_time(day: Day) {
    let now = chrono::Local::now().naive_local();
    if now.year() as u32 == day.year && now.day() < day.day {
        //We need to wait for the challenge to start
        let publish_time =
            chrono::NaiveDate::from_ymd(day.year as i32, 12, day.day).and_hms(0, 0, 0);
        let sleep_time = publish_time - now;
        info!("Challenge publishing in {:?}", sleep_time);
        //Sleep until 100 ms before the challenge comes out
        std::thread::sleep(sleep_time.to_std().unwrap());
        info!("Awoke for challenge");
    }
}

pub fn build_client(session: &str) -> Result<reqwest::blocking::Client, Error> {
    let jar = Arc::new(reqwest::cookie::Jar::default());
    jar.add_cookie_str(
        &format!("session={}", session),
        &"https://adventofcode.com".parse().unwrap(),
    );
    Ok(reqwest::blocking::Client::builder()
        .cookie_provider(jar)
        .build()
        .unwrap())
}

impl Problems {
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let _ = std::fs::remove_dir_all("./.problems");
        serde_fs::to_fs(&self, "./.problems")?;
        Ok(())
    }

    fn get(&self, day: Day) -> Option<&Data> {
        if let Some(years) = self.years.get(&day.year) {
            return years.get(&day.day);
        }
        None
    }

    pub fn lookup(&mut self, day: Day, part2_required: bool, tests_required: bool) -> Data {
        match self.try_lookup(day, part2_required, tests_required) {
            Ok(data) => data,
            Err(err) => panic!("Failed to download data for {}: {:?}", day, err),
        }
    }

    pub fn try_lookup(
        &mut self,
        day: Day,
        part2_required: bool,
        tests_required: bool,
    ) -> Result<Data, Error> {
        if let Some(data) = self.get(day).cloned() {
            //We already have the data we need
            if !part2_required || part2_required && data.part2.is_some() {
                return Ok(data);
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

            let (part1, part2) = parse_tests(&desc_body, day, tests_required)?;

            let years = self.years.entry(day.year).or_insert_with(|| HashMap::new());

            let data = years.entry(day.day).or_insert_with(|| {
                info!("Downloading input for {}", day);
                let input_url = format!(
                    "https://adventofcode.com/{}/day/{}/input",
                    day.year, day.day
                );
                let input = client.get(input_url).send().unwrap().text().unwrap();
                Data {
                    part1,
                    //This only runs the first time when part2_tests is None so thats why the clone
                    //is here
                    part2: part2.clone(),
                    input,
                }
            });
            data.part2 = part2;

            break Ok(data.clone());
        }
    }

    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(serde_fs::from_fs("./.problems")?)
    }

    pub fn new(session: String) -> Self {
        Self {
            years: HashMap::new(),
            session,
        }
    }
}
