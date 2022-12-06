use anyhow::{anyhow, Context, Result};
use log::*;
use parser::{AocPage, Client, Part, ProblemStageWithAnswers, TestCases};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::Entry;
use std::path::Path;

use std::collections::HashMap;

use crate::{AocDay, Input, Output};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Data {
    pub input: String,
    pub tests: TestCases,
    pub answers: ProblemStageWithAnswers,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct Day(pub u32);

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct Year(pub u32);

impl std::fmt::Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::fmt::Display for Year {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Problems {
    /// Mapping of days to problem data
    inner: HashMap<u32, Data>,
    pub session: String,
}

pub const DB_PATH: &str = "./.problems";

impl Problems {
    pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let _ = std::fs::remove_dir_all(path.as_ref());
        serde_fs::to_fs(&self, path)?;
        Ok(())
    }

    pub fn get(&self, day: Day) -> Option<&Data> {
        self.inner.get(&day.0)
    }

    pub fn get_mut(&mut self, day: Day) -> Option<&mut Data> {
        self.inner.get_mut(&day.0)
    }

    pub fn set(&mut self, day: Day, data: Data) {
        self.inner.insert(day.0, data);
    }

    /// Ensures that a given day's data is already cached and written to disk
    /// Returns `Ok` if the data is cached
    /// Returns `Err` if the given was not cached, and an error occured while downleading / parsing
    pub fn ensure_cached(&mut self, client: &mut Client, year: Year, day: Day) -> Result<()> {
        if self.inner.contains_key(&day.0) {
            // No work to do since, when a correct answer is submitted we grab the new tests
            // TODO: respect override flag
            return Ok(());
        }
        loop {
            match self.force_recache(client, year, day) {
                Ok(Some(_page)) => {
                    self.save(DB_PATH)?;
                    break Ok(());
                }
                Ok(None) => {
                    warn!("{year} day {day} not available yet");
                    // not availbe yet
                    std::thread::sleep(std::time::Duration::from_millis(500));
                }
                Err(e) => return Err(e),
            }
        }
    }

    /// Downleads `year` day `day` using `client` and stores the updated values into `self`
    /// Returns `Ok(None)` if its too early for the challenge
    pub fn force_recache(
        &mut self,
        client: &mut Client,
        year: Year,
        day: Day,
    ) -> Result<Option<AocPage>> {
        let body = client.download_problem(year.0, day.0)?;
        if body.contains(
            "the link will be enabled on the calendar the instant this puzzle becomes available",
        ) {
            warn!("sent request too early!");
            warn!("client or server clock out of sync?");
            //Try again in a little bit
            return Ok(None);
        }
        let day_contexte = || format!("parsing {} day {}", year, day);
        let page = AocPage::new(&body).with_context(day_contexte)?;

        match self.inner.entry(day.0) {
            Entry::Vacant(entry) => {
                let input = match page.embedded_puzzle_input() {
                    Some(i) => {
                        info!("using html embedded puzzle input: `{i}`");
                        i
                    }
                    None => {
                        info!("Downloading input for {}", day);
                        client
                            .download_input(year.0, day.0)
                            .with_context(day_contexte)?
                    }
                };

                entry.insert(Data {
                    input,
                    tests: page.test_cases().with_context(day_contexte)?,
                    answers: page.answers().with_context(day_contexte)?,
                });
            }
            Entry::Occupied(entry) => {
                entry.into_mut().merge(&page)?;
            }
        };
        Ok(Some(page))
    }

    pub fn load() -> anyhow::Result<Self> {
        serde_fs::from_fs(DB_PATH).with_context(|| format!("while loading {DB_PATH}"))
    }

    /// Deletes all old data and resets
    pub fn nuke(session: String) -> Result<Self> {
        let _ = std::fs::remove_dir_all(DB_PATH);
        let ret = Self {
            inner: HashMap::new(),
            session,
        };
        ret.save(DB_PATH)
            .with_context(|| format!("while saving to {DB_PATH}"))?;
        Ok(ret)
    }
}

impl Data {
    /// Tries to run the given part using `implementation`, but if no test cases are available this
    /// logs and returns None
    /// On success returns Some((test_result, expected_value))
    pub fn run_test(
        &self,
        implementation: &dyn AocDay,
        part: Part,
    ) -> Option<(Output, Option<String>)> {
        let test = match part {
            Part::Part1 => Some(self.tests.part1()),
            Part::Part2 => self.tests.part2(),
        };
        test.map(|test| -> Option<_> {
            let input = Input(test.clone().input?);
            let expected = test.output.clone();

            let out = match part {
                Part::Part1 => implementation.part1(input),
                Part::Part2 => implementation.part2(input),
            };
            Some((out, expected))
        })
        .flatten()
    }

    pub fn run(&self, implementation: &dyn AocDay, part: Part) -> Result<Output> {
        let input = Input(self.input.clone());
        Ok(match part {
            Part::Part1 => implementation.part1(input),
            Part::Part2 => implementation.part2(input),
        })
    }

    /// Merges the new values from `page` into `self`
    fn merge(&mut self, page: &AocPage) -> Result<()> {
        let mut errors = vec![];
        match page.test_cases() {
            Ok(new_tests) => self
                .tests
                .merge(&new_tests)
                .unwrap_or_else(|e| errors.push(e.context("failed to parse test cases"))),
            Err(e) => errors.push(e),
        }

        match page.answers() {
            Ok(new_answers) => self
                .answers
                .merge(&new_answers)
                .unwrap_or_else(|e| errors.push(e)),
            Err(e) => errors.push(e.context("failed to parse answers")),
        }
        match errors.len() {
            0 => Ok(()),
            1 => Err(errors.remove(0)).context("failed to merge page data"),
            _ => Err(anyhow!(
                "failed to merge page data due to mutiple errors: {errors:?}"
            )),
        }
    }
}
