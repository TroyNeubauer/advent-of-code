mod helper;
mod traits;
mod util;
mod years;

use util::{Day, Problems};

use traits::AocDay;
use years::*;

use chrono::Datelike;
use clap::Parser;
use log::*;

struct RunData {
    year: u32,
    day: u32,
    implementation: Box<dyn traits::AocDay>,
    run_part_1: bool,
    run_part_2: bool,
    auto_submit: bool,
    running_all: bool,
}

fn run(problems: &mut Problems, data: RunData) {
    let year = data.year;
    let day = data.day;

    // The days that are known to not have parsable tests
    // When we are running all tests then don't require tests on these days so that we don't hold up
    // running everything when tests are eventually not parsed from the prose
    const MANUAL_TEST_DAYS: [Day; 1] = [Day { year: 2020, day: 5 }];

    let mut tests_required = true;
    if data.running_all && MANUAL_TEST_DAYS.contains(&Day { year, day }) {
        tests_required = false;
    }

    let day_data = problems.lookup(Day { year, day }, data.run_part_2, tests_required);

    problems.save().unwrap();

    let run_part = |test: &Option<util::Test>, part1, name| {
        match test {
            None => {
                info!("Not running test for {} {}", day, name);
                if data.auto_submit {
                    error!("Refusing to auto submit without tests. Please fill in manually");
                    return;
                }
            }
            Some(test) => {
                let input = traits::Input::new(test.input.clone());
                let output = if part1 {
                    data.implementation.part1(input)
                } else {
                    data.implementation.part2(input)
                };

                let expected = test.expected_output.trim();
                let output = output.into_inner();
                info!("{} - {}", expected, &output);
                if expected != output {
                    if data.running_all {
                        error!(
                            "{} day {} {} test failed:\n  expected `{}`\n  real `{}`",
                            year, day, name, expected, output
                        );
                        return;
                    } else {
                        panic!(
                            "{} test failed:\n  expected `{}`\n  real `{}`",
                            name, expected, output
                        );
                    }
                }
                if data.auto_submit {
                    info!("{} test {} succeeded!", name, expected);
                }
            }
        }

        let input = traits::Input::new(day_data.input.clone());
        let output = if part1 {
            data.implementation.part1(input)
        } else {
            data.implementation.part2(input)
        };
        let answer = output.into_inner();
        println!("----------------------------------------");
        println!();
        println!("     {} day {} {} answer: {}", year, day, name, &answer);
        println!();
        println!("----------------------------------------");

        if data.auto_submit {
            let url = format!(
                "https://adventofcode.com/{}/day/{}/answer",
                data.year, data.day
            );
            info!("Submitting answer {} to {}", &answer, &url);

            let level = if part1 { "1" } else { "2" };
            let params = [("level", level), ("answer", answer.as_str())];
            let client = util::build_client(problems.session.as_str()).unwrap();

            let res = client.post(url).form(&params).send().unwrap();

            let text = res.text().unwrap();
            if text.contains("You don't seem to be solving the right level") {
                info!("Looks like this problem has already been submitted");
            } else {
                info!("Server returned unknown response: {}", &text);
                use rand::Rng;
                let num: u32 = rand::thread_rng().gen();
                std::fs::write(format!("/tmp/aoc_res_reply{}.html", num), &text).unwrap();
            }
            trace!("Server response: {}", text);
        }
    };
    if data.run_part_1 {
        run_part(&day_data.part1, true, "part1");
    }

    if data.run_part_2 {
        run_part(&day_data.part2, false, "part2");
    }
}

fn main() {
    env_logger::init();
    let mut opts: Opts = Opts::parse();
    if opts.run {
        if opts.day.is_some() && opts.year.is_some() {
            debug!("Run mode enabled with year and day specified");
            debug!(
                "Using user specified year and day, however answers will still be auto submitted"
            );
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
                debug!("Assuming tomorrow");
                day = tomorrow.day();
            }

            if opts.day.is_none() {
                debug!("Overriding day to be {}", day);
            }
            opts.day = Some(day);

            let year = now
                .year()
                .try_into()
                .expect("System time set to before CE!");
            if opts.year.is_none() {
                debug!("Overriding year to be {}", year);
            }
            opts.year = Some(year);
        }
    }

    let mut problems = match Problems::load() {
        Ok(p) => p,
        Err(err) => {
            warn!("Failed to load problems: {}", err);
            warn!("Creating new problems database");
            let session = opts
                .session
                .expect("--session must be given if no existing problems database exists!");

            Problems::new(session)
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
    let auto_submit = opts.run;

    let to_run = if opts.all {
        let mut to_run = Vec::new();
        let years = vec![(2020, 1..=16), (2021, 1..=4)];
        for (year, range) in years {
            for day in range {
                to_run.push(Day { year, day });
            }
        }
        to_run
    } else {
        let day = opts
            .day
            .expect("Day not supplied. Use --run, or set day with --day");
        let year = opts
            .year
            .expect("Year not supplied. Use --run, or set day with --year");
        debug!("Running year: {}, day {}", day, year);
        vec![Day { year, day }]
    };

    for day_info in to_run {
        let day = day_info.day;
        let year = day_info.year;
        //TODO: use a marco
        let implementation: Box<dyn AocDay> = match year {
            2020 => match day {
                1 => Box::new(y2020::day1::S),
                2 => Box::new(y2020::day2::S),
                3 => Box::new(y2020::day3::S),
                4 => Box::new(y2020::day4::S),
                5 => Box::new(y2020::day5::S),
                6 => Box::new(y2020::day6::S),
                7 => Box::new(y2020::day7::S),
                8 => Box::new(y2020::day8::S),
                9 => Box::new(y2020::day9::S),
                10 => Box::new(y2020::day10::S),
                11 => Box::new(y2020::day11::S),
                12 => Box::new(y2020::day12::S),
                13 => Box::new(y2020::day13::S),
                14 => Box::new(y2020::day14::S),
                15 => Box::new(y2020::day15::S),
                16 => Box::new(y2020::day16::S),
                _ => panic!("Unknown day {}, for year {}", day, year),
            },
            2021 => match day {
                1 => Box::new(y2021::day1::S),
                2 => Box::new(y2021::day2::S),
                3 => Box::new(y2021::day3::S),
                4 => Box::new(y2021::day4::S),
                5 => Box::new(y2021::day5::S),
                6 => Box::new(y2021::day6::S),
                7 => Box::new(y2021::day7::S),
                8 => Box::new(y2021::day8::S),
                _ => panic!("Unknown day {}, for year {}", day, year),
            },
            _ => panic!("Unknown year {}", year),
        };

        let data = RunData {
            year,
            day,
            implementation,
            run_part_1: run1,
            run_part_2: run2,
            auto_submit,
            running_all: opts.all,
        };

        run(&mut problems, data);
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

    /// Runs all problems, from all years. This overrides --run, --day, and --year
    #[clap(long)]
    all: bool,
}
