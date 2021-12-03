mod traits;
mod util;
mod years;

use traits::{AocDay, Input, Output};
use years::*;

use chrono::Datelike;
use clap::Parser;
use log::*;

fn run(
    problems: &mut util::Problems,
    year: u32,
    day: u32,
    implementation: impl traits::AocDay,
    run_part_1: bool,
    run_part_2: bool,
) {
    let day_data = problems.lookup(util::Day { year, day }, run_part_2);

    problems.save().unwrap();

    let run_part = |tests: &Vec<util::Test>, part1, name| {
        for test in tests.iter() {
            let input = traits::Input::new(test.input.clone());
            let output = if part1 {
                AocDay::part1(&implementation, input)
            } else {
                AocDay::part2(&implementation, input)
            };

            let expected = test.expected_output.trim();
            let output = output.into_inner();
            if expected != output {
                panic!(
                    "Test failed {}:\n  expected `{}`\n  real `{}`",
                    name, expected, output
                );
            }
            info!("{} test {} succeeded!", name, expected);
        }

        let input = traits::Input::new(day_data.input.clone());
        let output = if part1 {
            AocDay::part1(&implementation, input)
        } else {
            AocDay::part2(&implementation, input)
        };
        println!("----------------------------------------");
        println!();
        println!("     {} answer: {}", name, output.into_inner());
        println!();
        println!("----------------------------------------");
    };
    if run_part_1 {
        run_part(&day_data.part1_tests, true, "part1");
    }

    if run_part_2 {
        run_part(
            &day_data
                .part2_tests
                .expect("Cannot run part 2 tests without data"),
            false,
            "part2",
        );
    }
}

fn main() {
    env_logger::init();
    let mut opts: Opts = Opts::parse();
    if opts.run {
        if opts.day.is_some() && opts.year.is_some() {
            info!("Run mode enabled with year and day specified");
            info!("Using user specified year and day, however answers will still be auto submitted");
        } else {
            let now = chrono::Local::now();
            let now_naive = now.naive_local();
            let tomorrow = chrono::NaiveDate::from_ymd(
                now_naive.year(),
                now_naive.month(),
                now_naive.day() + 1,
            )
            .and_hms(0, 0, 0);

            let mut day = now.day();
            //If we are right before the next day, assume we want the next day
            if tomorrow - now_naive < chrono::Duration::minutes(5) {
                info!("Assuming tomorrow");
                day = tomorrow.day();
            }

            if opts.day.is_none() {
                info!("Overriding day to be {}", day);
            }
            opts.day = Some(day);

            let year = now
                .year()
                .try_into()
                .expect("System time set to before CE!");
            if opts.year.is_none() {
                info!("Overriding year to be {}", year);
            }
            opts.year = Some(year);
        }
    }

    let day = opts
        .day
        .expect("Day not supplied. Use --run, or set day with --day");
    let year = opts
        .year
        .expect("Year not supplied. Use --run, or set day with --year");
    info!("Running year: {}, day {}", day, year);

    let mut problems = match util::Problems::load() {
        Ok(p) => p,
        Err(err) => {
            warn!("Failed to load problems: {}", err);
            warn!("Creating new problems database");
            let session = opts
                .session
                .expect("--session must be given if no existing problems database exists!");

            util::Problems::new(session)
        }
    };

    let run1 = {
        if !opts.run {
            true
        } else {
            !opts.part2
        }
    };

    let run2 = {
        if !opts.run {
            if opts.part2 {
                warn!("Using --part2 while run mode is inactive has no effect");
                warn!("Both parts are run by default");
            }
            true
        } else {
            opts.part2
        }
    };

    //TODO: use a marco
    match year {
        2020 => match day {
            1 => run(&mut problems, year, day, y2020::day1::S, run1, run2),
            2 => run(&mut problems, year, day, y2020::day2::S, run1, run2),
            3 => run(&mut problems, year, day, y2020::day3::S, run1, run2),
            4 => run(&mut problems, year, day, y2020::day4::S, run1, run2),
            _ => panic!("Unknown day {}, for year {}", day, year),
        },
        2021 => match day {
            1 => run(&mut problems, year, day, y2021::day1::S, run1, run2),
            2 => run(&mut problems, year, day, y2021::day2::S, run1, run2),
            3 => run(&mut problems, year, day, y2021::day3::S, run1, run2),
            4 => run(&mut problems, year, day, y2021::day4::S, run1, run2),
            _ => panic!("Unknown day {}, for year {}", day, year),
        },
        _ => panic!("Unknown year {}", year),
    }

    problems.save().unwrap();
}

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Parser)]
#[clap(version = "1.0", author = "Troy Neubauer <troyneubauer@gmail.com>")]
struct Opts {
    /// The year to run
    #[clap(short, long)]
    year: Option<u32>,

    /// The problem to run
    #[clap(short, long)]
    day: Option<u32>,

    /// Activates 'run' mode where new puzzle input is downloaded and tests are run automatically
    /// to aid in development speed when competing
    #[clap(short, long)]
    run: bool,

    /// Runs the solution for part 2. Only meaningful in run mode, normally both solution are
    /// printed
    #[clap(short, long)]
    part2: bool,

    /// The value of the cookie `session` given by the advent of code server for authentication
    #[clap(long)]
    session: Option<String>,
}
