
mod util;
mod traits;

mod day1;
mod day2;

use chrono::Datelike;
use clap::Parser;
use log::*;

fn main() {
    env_logger::init();
    let mut opts: Opts = Opts::parse();
    if opts.run {
        let now = chrono::Local::now();
        let now_naive = now.naive_local();
        let tomorrow =
            chrono::NaiveDate::from_ymd(now_naive.year(), now_naive.month(), now_naive.day() + 1)
                .and_hms(0, 0, 0);

        let mut day = now.day();
        //If we are right before the next day, assume we want the next day
        if tomorrow - now_naive < chrono::Duration::hours(4) {
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

    let day = opts
        .day
        .expect("Day not supplied. Use --run, or set day with --day");
    let year = opts
        .year
        .expect("Year not supplied. Use --run, or set day with --year");
    info!("Running year: {}, day {}", day, year);

    let mut problems = util::Problems::new("53616c7465645f5f6203c384f07e9cc2617db3460d5d53b661952e5f45a1ff1125c25fb628c32313e0d43d85be024439".to_string());
    //let mut problems = util::Problems::load().unwrap();

    let data = problems.lookup(util::Day { year, day }, false);
    println!("{}", data.input);

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
}
