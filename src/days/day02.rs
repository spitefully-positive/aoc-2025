#![allow(dead_code, unreachable_code, unused_imports)]
use std::iter::Filter;

use crate::{Solution, SolutionPair};

/**
 * Part 1:
 * You get a comma seperated list of ID ranges, all on a single line.
 * A range looks like this "11-22", or like this "95-115".
 * You have to find all the funny ID's. A funny ID is an ID that consists only of one number,
 * or has like a repeating sequence of numbers. 11 and 22 are perfectly simple examples of that examples of that.
 * A more complicated one would be 6969, or something like 123123.
 *
 * None of the numbers have leading zeros. If you ever get a leading zero, you can throw that ID away.
 * Also note that the numbers shall only repeat twice. By that logic I think that 123123123 would not be a funny ID.
 *
 * On a range like "11-22" you have to search every ID between those two numbers.
 * 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21 and 22.
 *
 * Part 2:
 * Same but now a funny ID is not only ever repeated twice, but could also repeated 3, 5, etc... times
 * It could also be just 1's for example. (11111111111111111111111111 would be valid)
 */

pub fn check_id_part1(id: &u64) -> bool {
    let num_as_str = id.to_string();
    let len = num_as_str.chars().count();

    if len % 2 != 0 {
        return false;
    }

    let (left, right) = num_as_str.split_at(len / 2);
    if left == right {
        return true;
    } else {
        return false;
    }
}

pub fn check_id_part2(id: &u64) -> bool {
    let num_as_str = id.to_string();
    let len = num_as_str.chars().count();

    let mut snippet = String::from("");
    'outer: for char in num_as_str.chars() {
        snippet = format!("{}{}", snippet, char);
        let snip_len = snippet.chars().count();

        if snip_len > len / 2 || len % snip_len != 0 {
            continue;
        }

        // Do the check
        for (idx, char) in num_as_str.chars().enumerate() {
            if char != snippet.chars().nth(idx % snip_len).unwrap() {
                continue 'outer;
            }
        }

        return true;
    }

    return false;
}

pub fn solve() -> (u64, u64) {
    let input = Range::from_input(input());

    let mut solution_part1: u64 = 0;
    let mut solution_part2: u64 = 0;

    input
        .into_iter()
        .flat_map(|range| {
            let mut numbers: Vec<u64> = vec![];
            for number in range.start..=range.end {
                numbers.push(number);
            }
            return numbers;
        })
        .for_each(|num| {
            if check_id_part1(&num) {
                solution_part1 += num;
            }

            if check_id_part2(&num) {
                solution_part2 += num;
            }
        });

    return (solution_part1, solution_part2);
}

#[derive(Debug)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn from_input(input: &str) -> Vec<Range> {
        let ranges = input
            .split(",")
            .map(|pair| {
                let (start, end) = pair.split_once('-').unwrap();
                return Range {
                    start: start.trim().parse().unwrap(),
                    end: end.trim().parse().unwrap(),
                };
            })
            .collect::<Vec<Range>>();

        return ranges;
    }
}

#[cfg(test)]
fn input() -> &'static str {
    return "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
}

#[cfg(not(test))]
fn input() -> &'static str {
    return include_str!("./day02-input.txt");
}

#[test]
fn test() {
    assert_eq!(solve(), (1227775554, 4174379265));
}

pub fn get_solution() -> SolutionPair {
    let solved = solve();
    return (Solution::from(solved.0), Solution::from(solved.1));
}
