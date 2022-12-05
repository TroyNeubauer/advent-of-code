use crate::{problems, AocDay, Data, Day, Input, Output, Problems, Year};

use clap::Parser;
use log::*;
use parser::Client;

struct RunData<'a> {
    day: Day,
    year: Year,
    implementation: &'a dyn AocDay,
    auto_submit: bool,
}

enum RefreshStatus {
    /// The page must be re-chached
    RefreshRequired,
    None,
}

fn run(problems: &mut Problems, data: RunData) -> Result<()> {
    let year = data.year;
    let day = data.day;

    let mut client = Client::new(&problems.session).expect("failed to create client");
    problems.ensure_cached(&mut client, year, day)?;
    let day_data = problems.get(day).unwrap();

    /*
    let run_part = |day_data: &Data, part1, name| -> RefreshStatus {
        day.run_tes
        
                info!("test expected: {} got: {}", expected, &output);
                if expected != output {
                    panic!(
                        "{} test failed:\n  expected `{}`\n  real `{}`",
                        name, expected, output
                    );
                }
                if data.auto_submit {
                    info!("{} test {} succeeded!", name, expected);
                }
    */

        day_data.try_run
        let answer = output.into_inner();
        println!("----------------------------------------");
        println!();
        println!("     {} day {} {} answer: {}", year, day, name, &answer);
        println!();
        println!("----------------------------------------");

        if data.auto_submit {
            let url = format!("https://adventofcode.com/{}/day/{}/answer", year, day);
            info!("Submitting answer {} to {}", &answer, &url);

            let level = if part1 { "1" } else { "2" };
            let params = [("level", level), ("answer", answer.as_str())];
            let client = problems::build_client(problems.session.as_str()).unwrap();

            let res = client.post(url).form(&params).send().unwrap();

            let text = res.text().unwrap();
            let already_submitted = text.contains("You don't seem to be solving the right level");
            let correct = text.contains("That's the right answer!");
            let incorrect = text.contains("That's not the right answer.");

            if already_submitted || correct {
                let sol = if part1 {
                    info!("Advancing to part 2");
                    day_data.part = Part::Part2;
                    &mut day_data.p1_ans
                } else {
                    &mut day_data.p2_ans
                };
                info!("Saving {answer} as the correct answer");
                *sol = Some(Output(answer));
            }
            if already_submitted {
                info!("Looks like this problem has already been submitted");
            } else if correct {
                println!("That's the right answer!");
            } else if incorrect {
                println!("That's not the right answer.");
            } else {
                info!("Server returned unknown response: {}", &text);
                use rand::Rng;
                let num: u32 = rand::thread_rng().gen();
                let path = format!("/tmp/aoc_res_reply{}.html", num);
                std::fs::write(&path, &text).unwrap();
                info!("Wrote html reply dump to `{path}`");
            }
            trace!("Server response: {}", text);
    let both_solved = day_data.is_part1_solved() && day_data.is_part2_solved();
    let run_p1 = !data.auto_submit || !day_data.is_part1_solved() || both_solved;
    let run_p2 = !data.auto_submit || day_data.is_part1_solved() || both_solved;

    if run_p1 {
        run_part(&mut day_data, true, "part1");
    }

    if run_p2 {
        run_part(&mut day_data, false, "part2");
    }

    problems.store(data.day, day_data);

    problems.save().unwrap();
}

fn wait_for_time(day: Day) {
    let now_millis = chrono::Local::now().naive_local().timestamp_millis();
    // AOC releases at midnight in the eastern american timezone
    let est = chrono_tz::Tz::America__New_York;
    // We need to wait for the challenge to start
    let publish_millis = chrono::NaiveDate::from_ymd_opt(day.year as i32, 12, day.day)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_local_timezone(est)
        .unwrap()
        .timestamp_millis();

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
    env_logger::builder()
        .filter(None, LevelFilter::Info)
        .filter(Some("aoc"), LevelFilter::Trace)
        .init();
    let opts: Opts = Opts::parse();

    let mut problems = match opts.session {
        Some(session) => Problems::nuke(session).unwrap(),
        None => Problems::load().unwrap(),
    };

    let auto_submit = opts.run;

    debug!("Running year: {}, day {}", day, year);
    let day = Day { year, day };

    let data = RunData {
        day,
        implementation,
        auto_submit,
    };

    run(&mut problems, data);

    problems.save().unwrap();
}

#[derive(Parser)]
#[clap(version = "1.0", author = "Troy Neubauer <troyneubauer@gmail.com>")]
struct Opts {
    /// Activates 'run' mode where new puzzle input is downloaded and tests are run automatically
    /// to aid in development speed when competing
    #[clap(short, long)]
    run: bool,

    /// Stores the given session cookie to the problem database for auto-download and submit later
    #[clap(long)]
    session: Option<String>,
}
