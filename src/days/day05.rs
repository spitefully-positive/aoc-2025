#![allow(dead_code, unreachable_code, unused_imports)]
use clap::Parser;

use crate::{Solution, SolutionPair};
use std::{
    collections::HashSet,
    fmt::Display,
    hash::Hash,
    ops::{Range, RangeInclusive},
    str::FromStr,
    u64,
};

/**
 * Part 1:
 * You need to determine the amount of ingredients that are fresh.
 * Your input concists of two parts.
 *
 * First there are ID ranges e. G. 3-5 stands for ID's 3, 4, 5.
 * (Divided by new lines)
 *
 * Then one completely empty line.
 *
 * And then line by line a list of ID's
 *
 * The ID ranges descripe which of the listed ID's are considered fresh.
 * You need to count how many ID's are fresh, and return the count.
 * Curious to see where part two goes.
 * Part 2:
 *
 */
pub fn solve() -> (u64, u64) {
    let vec: Vec<&str> = input().lines().collect();

    let split_index = vec
        .iter()
        .position(|line| *line == "")
        .expect("Did not find the empty line");

    let (fresh_ranges, all_ids) = vec.split_at(split_index);

    let fresh_ranges: Vec<RangeInclusive<u64>> = fresh_ranges
        .into_iter()
        .map(|line| line_to_range_unchecked(line))
        .collect();
    // We need to deduplicate fresh ID's because the ranges overlap sometimes
    let fresh_ranges = merge_ranges(fresh_ranges);

    let all_ids: Vec<u64> = all_ids
        .into_iter()
        .skip(1) // Skrip the emtpy line from split
        .map(|line| {
            return line
                .trim()
                .parse::<u64>()
                .expect("I know every ID is u64 compatible");
        })
        .collect();

    let part1_solution = all_ids
        .iter()
        .filter(|curr_id| fresh_ranges.iter().any(|range| range.contains(curr_id)))
        .count() as u64;

    let part2_solution = fresh_ranges
        .into_iter()
        .fold(0u64, |acc, range| acc + range.count() as u64);

    return (part1_solution, part2_solution);
}

fn line_to_range_unchecked(s: &str) -> RangeInclusive<u64> {
    let (start, end) = s
        .trim()
        .split_once('-')
        .expect("Error while splitting ranges");

    return start
        .parse::<u64>()
        .expect("Couldnt convert string to id range start")
        ..=end
            .parse::<u64>()
            .expect("Couldnt convert string to id range end");
}

fn merge_ranges(ranges: Vec<RangeInclusive<u64>>) -> Vec<RangeInclusive<u64>> {
    let mut ranges = ranges
        .into_iter()
        .map(|range| (*range.start() as u64, *range.end() as u64))
        .collect::<Vec<_>>();
    ranges.sort();

    let mut merged_ranges = Vec::new();
    let mut maybe_last_range: Option<RangeInclusive<u64>> = None;

    for (start, end) in ranges {
        if let Some(last_range) = maybe_last_range {
            if start <= *last_range.end() {
                maybe_last_range = Some(*last_range.start()..=end.max(*last_range.end() as u64))
            } else {
                merged_ranges.push(last_range);
                maybe_last_range = Some(start..=end);
            }
        } else {
            maybe_last_range = Some(start..=end);
        }
    }

    if let Some(last_range) = maybe_last_range {
        merged_ranges.push(last_range);
    }

    return merged_ranges;
}

#[cfg(test)]
fn input() -> &'static str {
    return "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";
}

#[cfg(not(test))]
fn input() -> &'static str {
    return include_str!("./day05-input.txt");
}

#[test]
fn test() {
    assert_eq!(solve(), (3, 14));
}

pub fn get_solution() -> SolutionPair {
    let solved = solve();
    return (Solution::from(solved.0), Solution::from(solved.1));
}
