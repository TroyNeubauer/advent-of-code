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
    pub part1_tests: Vec<Test>,
    #[serde(rename = "p2")]
    pub part2_tests: Option<Vec<Test>>,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct Day {
    pub year: u32,
    pub day: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Problems {
    /// Mapping of years to days to problem data
    //TODO: Make better once serde_fs supports more types as keys
    years: HashMap<u32, HashMap<u32, Data>>,
    session: String,
}

fn parse_tests(
    desc_body: &str,
    part2_enabled: bool,
) -> Result<(Vec<Test>, Option<Vec<Test>>), Error> {
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

    let part2: Vec<_> = document.find(Name("h2").and(Attr("id", "part2"))).collect();
    if part2.len() > 1 {
        error!("Found multiple elements with the part2 tag");
        error!("Elements: {:?}", part2);
        std::process::exit(1);
    }
    let part2 = part2.get(0);

    if test_inputs.len() == 0 {
        error!("Unable to find examples for tests. Please fill in the files in `input-cache`");
        error!("Segments: {:?}", test_inputs);
        std::process::exit(1);
    }
    // Duplicate the test input if later tests are missing it
    // (most challenges share the same test input)
    for (i, test) in test_solutions.iter().enumerate() {
        if test_inputs.get(i).is_none() {
            if let Some(part2) = part2 {
                if test.index() < part2.index() {
                    error!("Found duplicate part 1 input!");
                    error!("inputs: {:?}", test_inputs);
                    error!("outputs: {:?}", test_solutions);
                    std::process::exit(1);
                }
            }
            debug_assert_eq!(i, test_inputs.len());
            let to_add = test_inputs[test_inputs.len() - 1].clone();
            info!("Duplicating input: `{}`", to_add.text());
            test_inputs.push(to_add);
        }
    }

    if test_solutions.is_empty() || part2_enabled && test_solutions.len() == 1 {
        warn!(
            "Unable to find solutions for tests. Solutions: {:?}",
            test_solutions
        );
    }

    debug!("test_inputs: {:?}", test_inputs);
    debug!("test_solutions: {:?}", test_solutions);

    let mut part1_tests = Vec::new();
    let mut part2_tests = Vec::new();

    test_inputs
        .into_iter()
        .zip(test_solutions)
        .for_each(|(input, output)| {
            //Make sure that the output comes after the input
            assert!(input.index() < output.index());
            let test = Test {
                input: input.text(),
                expected_output: output.text(),
            };
            match part2 {
                //If we know where part 2 starts in the html, and we are after it, then this is a
                //part 2 test
                Some(part) if output.index() > part.index() => part2_tests.push(test),
                _ => part1_tests.push(test),
            }
        });

    let part2_tests = if part2_tests.is_empty() {
        None
    } else {
        Some(part2_tests)
    };
    Ok((part1_tests, part2_tests))
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

    pub fn lookup(&mut self, day: Day, part2_required: bool) -> Data {
        if let Some(data) = self.get(day).cloned() {
            //We already have the data we need
            if !part2_required || part2_required && data.part2_tests.is_some() {
                info!("We already have the data for this stage of the problem");
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

            let (part1_tests, part2_tests) = parse_tests(&desc_body, part2_required).unwrap();

            let years = self.years.entry(day.year).or_insert_with(|| HashMap::new());

            let data = years.entry(day.day).or_insert_with(|| {
                info!("Downloading input for year {} day {}", day.year, day.day);
                let input_url = format!(
                    "https://adventofcode.com/{}/day/{}/input",
                    day.year, day.day
                );
                let input = client.get(input_url).send().unwrap().text().unwrap();
                Data {
                    part1_tests,
                    //This only runs the first time when part2_tests is None so thats why the clone
                    //is here
                    part2_tests: part2_tests.clone(),
                    input,
                }
            });
            data.part2_tests = part2_tests;

            break data.clone();
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

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
