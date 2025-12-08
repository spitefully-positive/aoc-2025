mod days;
mod etc;

use clap::Parser;
use etc::Solution;
use etc::cli::Args;
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

fn get_day_solver(day: u8) -> fn() -> SolutionPair {
    match day {
        1 => days::day01::get_solution,
        2 => days::day02::get_solution,
        3 => days::day03::get_solution,
        4 => days::day04::get_solution,
        5 => days::day05::get_solution,
        6 => days::day06::get_solution,
        7 => days::day07::get_solution,
        8 => days::day08::get_solution,
        9 => days::day09::get_solution,
        10 => days::day10::get_solution,
        11 => days::day11::get_solution,
        12 => days::day12::get_solution,
        13 => days::day13::get_solution,
        14 => days::day14::get_solution,
        15 => days::day15::get_solution,
        16 => days::day16::get_solution,
        17 => days::day17::get_solution,
        18 => days::day18::get_solution,
        19 => days::day19::get_solution,
        20 => days::day20::get_solution,
        21 => days::day21::get_solution,
        22 => days::day22::get_solution,
        23 => days::day23::get_solution,
        24 => days::day24::get_solution,
        25 => days::day25::get_solution,
        _ => panic!("Day {} was not even considered to be implemented", day),
    }
}
