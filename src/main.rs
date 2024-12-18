#![cfg_attr(test, feature(test))]

#[cfg(test)]
extern crate test;

mod scaffold;
mod utils;

use std::time::{Duration, Instant};

use crate::scaffold::{AocDay, AocResult, AocYear, get_input};

use clap::*;
use scaffold::InputParser;

aoc_solvers!{
    year_2015 {
        day_01,
        day_02,
        day_03,
        day_04,
        day_05
    },
    year_2022 {
        day_01,
        day_02,
        day_03,
        day_04,
        day_05,
        day_06,
        day_07,
        day_08,
        day_09,
        day_10,
        day_11,
        day_12,
        day_13,
        day_14,
        day_15,
        day_16,
        day_17,
        day_18,
        day_19,
        day_20,
        day_21,
        day_22,
        day_23,
        day_24,
        day_25
    },
    year_2024 {
        day_01,
        day_02,
        day_03,
        day_04,
        day_05,
        day_06,
        day_07,
        day_08,
        day_09,
        day_10,
        day_11,
        day_12,
        day_13,
        day_14,
        day_15,
        day_16,
        day_17,
        day_18
    }
}

#[derive(Parser)]
struct AocArgs {
    #[command(subcommand)]
    command: Option<AocCommand>
}

#[derive(Subcommand)]
enum AocCommand {
    List,
    Run(RunCommand)
}

impl Default for AocCommand {

    fn default() -> Self {
        AocCommand::Run(RunCommand::default())
    }

}

#[derive(Args, Clone, Default)]
struct RunCommand {
    #[arg(short, long)]
    year: Option<String>,

    #[arg(short, long, requires = "year")]
    day: Option<String>,

    #[arg(short, long)]
    example: bool
}

fn run_solver(aoc_day: &AocDay, input: String) -> Duration {
    let part_1 = {
        let start = Instant::now();
        let answer = (aoc_day.part_1)(InputParser::new(&input));
        let end = Instant::now();

        (answer, end - start)
    };

    let part_2 = {
        let start = Instant::now();
        let answer = (aoc_day.part_2)(InputParser::new(&input));
        let end = Instant::now();

        (answer, end - start)
    };

    println!("  - {}:", aoc_day.day);
    println!("    - Part 1 ({:?}) = {}", part_1.1, part_1.0);
    println!("    - Part 2 ({:?}) = {}", part_2.1, part_2.0);

    part_1.1 + part_2.1
}

fn aoc_main(aoc_years: &[AocYear]) {
    let mut args = AocArgs::parse();
    match args.command.take().unwrap_or_default() {
        AocCommand::List => {
            for aoc_year in aoc_years {
                println!("{}:", aoc_year.year);
                for aoc_day in aoc_year.days {
                    println!(" - {}", aoc_day.day);
                }
            }

        },
        AocCommand::Run(args) => {
            let mut total_duration = Duration::default();
            for aoc_year in aoc_years {
                if let Some(year) = &args.year {
                    if aoc_year.year != year {
                        continue;
                    }
                }

                println!("{}:", aoc_year.year);
                for aoc_day in aoc_year.days {
                    if let Some(day) = &args.day {
                        if aoc_day.day != day {
                            continue;
                        }
                    }
                    
                    total_duration +=
                        run_solver(
                            aoc_day,
                            get_input(aoc_year.year, aoc_day.day, args.example));
                }
            }

            println!("Total duration {:?}", total_duration);
        }
    }
}
