use crate::{problems::DB_PATH, AocDay, Data, Day, Problems, Year};
use std::{ops::BitOrAssign, time::Duration as StdDuration};

use anyhow::Result;
use clap::Parser;
use log::*;
use parser::{Client, Part, ProblemStageWithAnswers, SubmitStatus};

struct RunData<'a> {
    day: Day,
    year: Year,
    implementation: &'a dyn AocDay,
    auto_submit: bool,
}

#[derive(Copy, Clone, Debug)]
enum RefreshStatus {
    /// The page must be re-chached
    RefreshRequired,
    NotRequired,
}

impl BitOrAssign for RefreshStatus {
    fn bitor_assign(&mut self, rhs: Self) {
        use RefreshStatus::*;
        *self = match (*self, rhs) {
            (RefreshRequired, RefreshRequired) => RefreshRequired,
            (RefreshRequired, NotRequired) => RefreshRequired,
            (NotRequired, RefreshRequired) => RefreshRequired,
            (NotRequired, NotRequired) => NotRequired,
        };
    }
}

fn run(problems: &mut Problems, data: RunData) -> Result<()> {
    let year = data.year;
    let day = data.day;

    let implementation = data.implementation;
    let mut client = Client::new(&problems.session).expect("failed to create client");
    problems.ensure_cached(&mut client, year, day)?;
    let day_data = problems.get_mut(day).unwrap();

    let mut run_part = |day_data: &mut Data, part| -> Result<RefreshStatus> {
        if let Some((output, expected)) = day_data.run_test(implementation, part) {
            if let Some(expected) = expected {
                if expected.as_str() == output.as_str() {
                    info!("day {} part {} test succeeded!", day, part);
                    info!("expected {} got {}", expected.as_str(), output.as_str());
                } else {
                    panic!(
                        "{} test failed:\n  expected `{}`\n  real `{}`",
                        part,
                        expected,
                        output.as_str()
                    );
                }
            } else {
                info!("{} test: {}", part, output.as_str());
            }
        };

        let answer = day_data.run(implementation, part)?;
        println!("----------------------------------------");
        println!();
        println!("     {} day {} answer: {}", year, day, answer.as_str());
        println!();
        println!("----------------------------------------");
        println!();

        if data.auto_submit {
            match client.submit(year.0, day.0, part, answer.as_str())? {
                SubmitStatus::AlreadySubmitted => println!("Problem already submitted"),
                SubmitStatus::Correct => {
                    println!("CORRECT");
                    // refresh because part 2 test cases are now available
                    return Ok(RefreshStatus::RefreshRequired);
                }
                SubmitStatus::Incorrect => println!("Incorrect"),
                SubmitStatus::Unknown(s) => {
                    println!("unknown server responce: {s}");
                    use rand::Rng;
                    let num: u32 = rand::thread_rng().gen();
                    let path = format!("/tmp/aoc_res_reply{}.html", num);
                    std::fs::write(&path, s)?;
                    info!("Wrote html reply dump to `{path}`");
                }
            }
        }
        Ok(RefreshStatus::NotRequired)
    };
    let (run_p1, run_p2) = if data.auto_submit {
        match day_data.answers {
            ProblemStageWithAnswers::Part1 => (true, false),
            ProblemStageWithAnswers::Part2 { .. } => (false, true),
            ProblemStageWithAnswers::Complete { .. } => (true, true),
        }
    } else {
        (true, true)
    };

    let mut refresh = RefreshStatus::NotRequired;
    if run_p1 {
        refresh |= run_part(day_data, Part::Part1)?;
    }

    if run_p2 {
        refresh |= run_part(day_data, Part::Part2)?;
    }

    if matches!(refresh, RefreshStatus::RefreshRequired) {
        problems.force_recache(&mut client, year, day)?;
    }
    problems.save(DB_PATH).unwrap();

    Ok(())
}

fn wait_for_time(year: Year, day: Day) {
    let now_millis = chrono::Local::now().naive_local().timestamp_millis();
    // AOC releases at midnight in the eastern american timezone
    let est = chrono_tz::Tz::America__New_York;
    // We need to wait for the challenge to start
    let publish_millis = chrono::NaiveDate::from_ymd_opt(year.0 as i32, 12, day.0 as u32)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_local_timezone(est)
        .unwrap()
        .timestamp_millis();

    dbg!(publish_millis, now_millis);
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

pub fn runner_main(implementation: &dyn AocDay, year: u32, day: u32) {
    let year = Year(year);
    let day = Day(day);
    env_logger::builder()
        .filter(None, LevelFilter::Info)
        .filter(Some("util"), LevelFilter::Trace)
        .init();
    let opts: Opts = Opts::parse();

    let mut problems = match opts.session {
        Some(session) => Problems::nuke(session).unwrap(),
        None => {
            let Ok(p) = Problems::load() else {
                println!("no existing problem database found");
                println!("run using `--session` or `-s` to setup database");
                return;
            };
            p
        }
    };

    let auto_submit = opts.run;

    debug!("Running year: {}, day {}", year, day);

    if auto_submit {
        //wait_for_time(year, day);
    }

    let data = RunData {
        day,
        year,
        implementation,
        auto_submit,
    };

    if let Err(e) = run(&mut problems, data) {
        println!("error: {e:?}");
    }

    problems.save(DB_PATH).unwrap();
}

#[derive(Parser)]
#[clap(version = "1.0", author = "Troy Neubauer <troyneubauer@gmail.com>")]
struct Opts {
    /// Activates 'run' mode where new puzzle input is downloaded and tests are run automatically
    /// to aid in development speed when competing
    #[clap(short, long)]
    run: bool,

    /// Stores the given session cookie to the problem database for auto-download and submit later
    #[clap(short, long)]
    session: Option<String>,
}
