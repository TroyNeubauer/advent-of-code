mod day1;
mod day2;
mod util;

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

        //Try do download puzzel input if it doesnt exist already
    }

    let day = opts
        .day
        .expect("Day not supplied. Use --run, or set day with --day");
    let year = opts
        .year
        .expect("Year not supplied. Use --run, or set day with --year");
    info!("Running year: {}, day {}", day, year);

    let data = util::save_data(year, day, false);
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
