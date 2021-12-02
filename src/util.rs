use chrono::{Datelike, Duration};
use log::*;
use select::document::Document;
use select::node::Node;
use select::predicate::{Name, Predicate};

use std::collections::HashMap;
use std::io;
use std::path::PathBuf;
use std::sync::Arc;

pub struct Test {
    input: String,
    expected_output: String,
}

pub struct Data {
    input: String,
    tests: Vec<Test>,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Day {
    pub year: u32,
    pub day: u32,
}

pub struct Problems(HashMap<Day, Data>);

impl Problems {
}

fn get_save_dir(day: Day) -> Result<PathBuf, io::Error> {
    let mut folder_path = PathBuf::from("./problems");
    folder_path.push(day.year.to_string());
    folder_path.push(day.day.to_string());
    if std::fs::metadata(&folder_path).is_err() {
        std::fs::create_dir_all(&folder_path)?;
    }
    Ok(folder_path)
}

impl Data {
    fn save(&self, day: Day) -> Result<(), io::Error> {
        let mut path = get_save_dir(day)?;

        path.push("input.txt");
        std::fs::write(&path, &self.input)?;
        path.pop();

        for (i, test) in self.tests.iter().enumerate() {
            path.push(format!("i{}.txt", i + 1));
            std::fs::write(&path, &test.input)?;
            path.pop();

            path.push(format!("o{}.txt", i + 1));
            std::fs::write(&path, &test.expected_output)?;
            path.pop();
        }

        Ok(())
    }
}

pub fn save_data(day: Day, part_2: bool) -> Data {
    let now = chrono::Local::now().naive_local();
    if now.year() as u32 == day.year && now.day() < day.day {
        //We need to wait for the challenge to start
        let publish_time = chrono::NaiveDate::from_ymd(day.year as i32, 12, day).and_hms(0, 0, 0);
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

    let session = String::from_utf8(std::fs::read("./problems/session").unwrap()).unwrap();
    let jar = Arc::new(reqwest::cookie::Jar::default());
    jar.add_cookie_str(
        &format!("session={}", session),
        &"https://adventofcode.com".parse().unwrap(),
    );
    let client = reqwest::blocking::Client::builder()
        .cookie_provider(jar)
        .build()
        .unwrap();

    let description_url = format!("https://adventofcode.com/{}/day/{}", day.year, day.day);
    let desc_body = client.get(description_url).send().unwrap().text().unwrap();

    let document = Document::from(desc_body.as_str());

    let mut test_inputs = Vec::new();
    for node in document.find(Name("pre").descendant(Name("code"))) {
        let children: Vec<_> = node.children().collect();
        if children.len() == 1 && children[0].children().collect::<Vec<_>>().len() == 0 {
            test_inputs.push(node.text());
        }
    }
    let mut folder_path = get_save_dir(day).unwrap();

    folder_path.push("index.html");
    std::fs::write(&folder_path, desc_body).expect("Failed to write problem index.html");
    folder_path.pop();

    let test_solutions: Vec<String> = document
        .find(Name("em"))
        .filter_map(|s| s.text().parse::<u32>().ok().map(|_| s.text()))
        .collect();

    if test_solutions.is_empty() || part_2 && test_solutions.len() == 1 {
        println!(
            "Unable to find solutions for tests. Solutions: {:?}",
            test_solutions
        );
        std::process::exit(1);
    }

    if test_inputs.len() != 1 {
        println!("Unable to find examples for tests. Please fill in the files in `input-cache`");
        println!("Segments: {:?}", test_inputs);
        std::process::exit(1);
    }

    let tests = test_inputs
        .into_iter()
        .zip(test_solutions)
        .map(|(i, o)| {
            //Make sure that the output comes after the input
            println!("{:?} {:?}", i, o);
            Test {
                input: i,
                expected_output: o,
            }
        })
        .collect();

    let data = Data {
        tests,
        input: todo!(),
    };
    data.save(day).expect("Failed to save data");
    data
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
