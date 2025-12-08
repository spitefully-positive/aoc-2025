mod cli;
mod days;
mod etc;

use clap::Parser;
use cli::Args;
use etc::Solution;
use std::time::{Duration, Instant};

fn main() {
    let args = Args::parse();

    let days_to_solve: Vec<u8> = args
        .days
        .into_iter()
        .filter(|day| *day >= 1 && *day <= 25)
        .collect();

    if days_to_solve.is_empty() {
        println!(
            "Did you forget to specify a day?\nWe threw away the invalid ones that were not between 1 and 25 :D"
        );
        return;
    }

    let display_total_runtime = days_to_solve.iter().count() > 0;
    let mut total_runtime: Duration = Duration::ZERO;

    for day in days_to_solve {
        println!();
        println!("Solving day {day}...");

        let start_time = Instant::now();
        let (part1, part2) = get_day_solver(day)();
        let elapsed_time = start_time.elapsed();
        total_runtime += elapsed_time;

        println!("===================================");
        println!("Part 1: {}", part1);
        println!("Part 2: {}", part2);
        println!("-----------------------------------");
        println!(
            "Took {:.4} ms",
            elapsed_time.as_nanos() as f64 / 1_000_000.0
        );
        println!("===================================");
        println!();
    }

    if display_total_runtime {
        println!("===================================");
        println!(
            "Total runtime: {:.4} ms",
            total_runtime.as_nanos() as f64 / 1_000_000.0
        );
        println!("===================================");
        println!();
    }
}

pub type SolutionPair = (Solution, Solution);

use days::{
    day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13, day14,
    day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25,
};

fn get_day_solver(day: u8) -> fn() -> SolutionPair {
    match day {
        1 => days::day01::get_solution,
        2 => day02::solve,
        3 => day03::solve,
        4 => day04::solve,
        5 => day05::solve,
        6 => day06::solve,
        7 => day07::solve,
        8 => day08::solve,
        9 => day09::solve,
        10 => day10::solve,
        11 => day11::solve,
        12 => day12::solve,
        13 => day13::solve,
        14 => day14::solve,
        15 => day15::solve,
        16 => day16::solve,
        17 => day17::solve,
        18 => day18::solve,
        19 => day19::solve,
        20 => day20::solve,
        21 => day21::solve,
        22 => day22::solve,
        23 => day23::solve,
        24 => day24::solve,
        25 => day25::solve,
        _ => panic!("Day {} was not even considered to be implemented", day),
    }
}
