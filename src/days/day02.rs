#![allow(dead_code, unreachable_code, unused_imports)]
use crate::{Solution, SolutionPair};
use std::u64;

/**
 * Part 1:
 *
 * Part 2:
 *
 */

pub fn solve() -> (u64, u64) {
    todo!("You did not implement this day yet");

    let sol1: u64 = 0;
    let sol2: u64 = 0;

    return (sol1, sol2);
}

#[cfg(test)]
fn input() -> &'static str {
    return "Your test input";
}

#[cfg(not(test))]
fn input() -> &'static str {
    // return include_str!("./day0X-input.txt");
    return "";
}

#[test]
fn test() {
    assert_eq!(solve(), (0, 0));
}

pub fn get_solution() -> SolutionPair {
    let solved = solve();
    return (Solution::from(solved.0), Solution::from(solved.1));
}
