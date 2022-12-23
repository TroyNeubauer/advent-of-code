use std::sync::Arc;

use anyhow::Result;
use log::{debug, trace};

pub struct Client {
    inner: reqwest::blocking::Client,
}

const BASE_URL: &str = "https://adventofcode.com";

pub enum SubmitStatus {
    AlreadySubmitted,
    Correct,
    Incorrect,
    Unknown(String),
}

#[derive(Copy, Clone, Debug)]
pub enum Part {
    Part1,
    Part2,
}

impl std::fmt::Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Part::Part1 => "part 1",
            Part::Part2 => "part 2",
        })
    }
}

impl Client {
    /// Creates a new client for performing actions with the aoc server using the given session key
    pub fn new(session: &str) -> Result<Self> {
        let jar = Arc::new(reqwest::cookie::Jar::default());
        let url = BASE_URL.parse()?;
        jar.add_cookie_str(&format!("session={session}"), &url);

        let inner = reqwest::blocking::Client::builder()
            .cookie_provider(jar)
            .build()?;
        Ok(Self { inner })
    }

    /// Downloads the problem html page for the given day from the aoc server
    pub fn download_problem(&mut self, year: u32, day: u32) -> Result<String> {
        debug!("downloading page for {year} day {day}");
        let url = format!("{BASE_URL}/{year}/day/{day}");
        Ok(self.inner.get(url).send()?.text()?)
    }

    /// Downloads the puzzle input for the given day from the aoc server
    pub fn download_input(&mut self, year: u32, day: u32) -> Result<String> {
        debug!("downloading input for {year} day {day}");
        let url = format!("{BASE_URL}/{year}/day/{day}/input",);
        trace!("url: {url}");
        Ok(self.inner.get(url).send()?.text()?)
    }

    /// Downloads the puzzle input for the given day from the aoc server
    pub fn submit(
        &mut self,
        year: u32,
        day: u32,
        part: Part,
        answer: &str,
    ) -> Result<SubmitStatus> {
        let url = format!("{BASE_URL}/{year}/day/{day}/answer");
        trace!("url: {url}");

        let level = match part {
            Part::Part1 => "1",
            Part::Part2 => "2",
        };

        let params = [("level", level), ("answer", &answer)];
        trace!("sending form: {params:?}");

        let res = self.inner.post(url).form(&params).send()?;

        let text = res.text()?;
        trace!("server response: {text}");
        Ok(
            if text.contains("You don't seem to be solving the right level") {
                SubmitStatus::AlreadySubmitted
            } else if text.contains("That's the right answer!") {
                SubmitStatus::Correct
            } else if text.contains("That's not the right answer") {
                SubmitStatus::Incorrect
            } else {
                SubmitStatus::Unknown(text)
            },
        )
    }
}
