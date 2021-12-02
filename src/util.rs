use chrono::{Datelike, Duration};
use log::*;
use select::document::Document;
use select::node::Node;
use select::predicate::{Attr, Class, Name, Predicate, Text};
use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::sync::Arc;

use crate::traits::Error;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Test {
    pub input: String,
    pub expected_output: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Data {
    pub input: String,
    pub part1_tests: Vec<Test>,
    pub part2_tests: Option<Vec<Test>>,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct Day {
    pub year: u32,
    pub day: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Problems {
    days: HashMap<Day, Data>,
    session: String,
}

fn parse_tests(desc_body: &str, part2: bool) -> Result<(Vec<Test>, Option<Vec<Test>>), Error> {
    let document = Document::from(desc_body);

    let mut test_inputs = Vec::new();
    for node in document.find(Name("pre").descendant(Name("code"))) {
        let children: Vec<_> = node.children().collect();
        if children.len() == 1 && children[0].children().next().is_none() {
            test_inputs.push(node);
        }
    }

    let test_solutions: Vec<Node> = document
        .find(Name("em"))
        .filter_map(|s| s.text().parse::<u32>().ok().map(|_| s))
        .collect();

    if test_solutions.is_empty() || part2 && test_solutions.len() == 1 {
        error!(
            "Unable to find solutions for tests. Solutions: {:?}",
            test_solutions
        );
        std::process::exit(1);
    }

    if test_inputs.len() != 1 {
        error!("Unable to find examples for tests. Please fill in the files in `input-cache`");
        error!("Segments: {:?}", test_inputs);
        std::process::exit(1);
    }
    let part2: Vec<_> = document.find(Name("h2").and(Attr("id", "part2"))).collect();
    if part2.len() > 1 {
        error!("Found multiple elements with the part2 tag");
        error!("Elements: {:?}", part2);
        std::process::exit(1);
    }
    let part2 = part2.get(0);

    println!("test_inputs: {:?}", test_inputs);
    println!("test_solutions: {:?}", test_solutions);
    println!("part2: {:?}", part2);

    let filter

    match part2 {
        Some(part2) => Err("oeu".into()),
        None => {
            let part1_tests = test_inputs
                .into_iter()
                .zip(test_solutions)
                .map(|(i, o)| {
                    //Make sure that the output comes after the input
                    println!("{:?} {:?}", i, o);
                    Test {
                        input: i.text(),
                        expected_output: o.text(),
                    }
                })
                .collect();

            Ok((part1_tests, None))
        }
    }
}

fn wait_for_time(day: Day) {
    let now = chrono::Local::now().naive_local();
    if now.year() as u32 == day.year && now.day() < day.day {
        //We need to wait for the challenge to start
        let publish_time =
            chrono::NaiveDate::from_ymd(day.year as i32, 12, day.day).and_hms(0, 0, 0);
        let sleep_time = publish_time - now;
        println!("Challenge published in {:?}", sleep_time);
        //Sleep until 100 ms before the challenge comes out
        std::thread::sleep((sleep_time - Duration::milliseconds(100)).to_std().unwrap());
        info!("Awoke for challenge");
        loop {
            let now = chrono::Local::now().naive_local();
            if now > publish_time {
                break;
            }
            //Spin loop until its time
            std::hint::spin_loop();
        }
    }
}

fn build_client(session: &str) -> Result<reqwest::blocking::Client, Error> {
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
        let mut s = flexbuffers::FlexbufferSerializer::new();
        self.serialize(&mut s)?;

        Ok(())
    }

    pub fn lookup(&mut self, day: Day, part2: bool) -> Data {
        if let Some(data) = self.days.get(&day).cloned() {
            //We already have the data we need
            if !part2 || part2 && data.part2_tests.is_some() {
                return data;
            }
        }
        loop {
            wait_for_time(day);
            let client = build_client(self.session.as_str()).unwrap();

            let description_url = format!("https://adventofcode.com/{}/day/{}", day.year, day.day);
            let desc_body = client.get(description_url).send().unwrap().text().unwrap();

            if desc_body.contains("the link will be enabled on the calendar the instant this puzzle becomes available") {
                //Try again in a little bit
                std::thread::sleep(std::time::Duration::from_millis(500));
                continue;
            }

            let (part1_tests, part2_tests) = parse_tests(&desc_body, part2).unwrap();

            let data = self.days.entry(day).or_insert_with(|| {
                let input_url = format!(
                    "https://adventofcode.com/{}/day/{}/input",
                    day.year, day.day
                );
                let input = client.get(input_url).send().unwrap().text().unwrap();
                Data {
                    part1_tests,
                    part2_tests: part2_tests.clone(),
                    input,
                }
            });
            data.part2_tests = part2_tests;

            break data.clone();
        }
    }

    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let bytes = std::fs::read("./problems.data")?;
        let reader = flexbuffers::Reader::get_root(bytes.as_slice())?;

        Ok(Self::deserialize(reader)?)
    }

    pub fn new(session: String) -> Self {
        Self {
            days: HashMap::new(),
            session,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
