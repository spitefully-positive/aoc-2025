#![allow(dead_code, unreachable_code, unused_imports)]
use crate::{Solution, SolutionPair};
use std::{fmt::format, u64};

/**
 * Part 1:
 * You get banks of batteries as input.
 * A bank is just a big number. Banks are seperated by \n.
 *
 * Each digit of a number (bank) represents a battery and its voltage.
 * You now need to pick exactly two batteries, and "add" their voltage.
 * The goal is to pick the biggest voltage
 *
 * But "adding" means just sticking the digits together, meaning 1 + 2 = 12 or 6 + 9 = 69
 * And you can not rearrange the numbers.
 * Meaning the biggest number here "811111111111119" is 89. You can not get 98.
 *
 * Part 2:
 * The change for part 2 is, that instead of selecting just 2 numbers, you can now select 12.
 * That makes it a LOT more complex in my opinion...
 *
 * Actually, I think I got it now
 */

/// Takes in a battery bank, and returns the max charge
fn solve_part1(battery_bank: &BatteryBank) -> u64 {
    let battery_bank = &battery_bank.battery_charges;
    let mut first_biggest = get_biggest(battery_bank, None).unwrap();

    // If its at the end of the bank, we cannot use it as our first digit
    if first_biggest.0 == battery_bank.len() - 1 {
        first_biggest = get_biggest(&battery_bank[..battery_bank.len() - 1], None).unwrap();
    }

    let second_biggest = get_biggest(battery_bank, Some(first_biggest.0)).unwrap();

    return format!("{}{}", first_biggest.1, second_biggest.1)
        .parse()
        .unwrap();
}

/// Returns index and value of the biggest value, that is still smaller than the parameter. Returns nothing, if there is nothing bigger, or there are no elements
fn get_biggest(vec: &[u8], after: Option<usize>) -> Option<(usize, u8)> {
    return vec
        .iter()
        .enumerate()
        .filter(|(idx, _)| {
            if let Some(after) = after {
                return *idx > after;
            } else {
                return true;
            }
        })
        .rev() // max only gets the last last biggest element, we want the first biggest element
        .max_by_key(|(_, val)| **val)
        .map(|(idx, val)| (idx, *val));
}

fn solve_part2(battery_bank: &BatteryBank) -> u64 {
    return NotFullBank::new()
        .fill_bank(&battery_bank.battery_charges)
        .iter()
        .fold(String::from(""), |acc, num| {
            return format!("{}{}", acc, num);
        })
        .parse()
        .expect("Failed when bank from string back to u64");
}

struct NotFullBank {
    curr_idx: usize,
    charges: [u8; 12],
}

impl NotFullBank {
    fn new() -> NotFullBank {
        return NotFullBank {
            curr_idx: 0,
            charges: [0; 12],
        };
    }

    /// Recursively fills the bank, and returns the maximum possible batteries
    fn fill_bank(mut self, buffer_to_pick_from: &[u8]) -> [u8; 12] {
        // If we are already done, consider this our happy little accident, and return yourself as FullBank
        if self.curr_idx >= self.charges.len() {
            return self.charges;
        }

        let (part_that_interests_us, _) = buffer_to_pick_from
            .split_at_checked(buffer_to_pick_from.len() - (self.charges.len() - 1 - self.curr_idx))
            .expect("Messed something up when splitting the buffer in next_pick");

        let (where_i_found_it, next_pick_value) = part_that_interests_us.iter().enumerate().fold(
            (0usize, 0u8),
            |biggest_yet, current| {
                if biggest_yet.1 < *current.1 {
                    return (current.0, *current.1);
                } else {
                    return biggest_yet;
                }
            },
        );

        // Set the new value in a safe way
        self.charges
            .get_mut(self.curr_idx)
            .map(|val| *val = next_pick_value);

        // And increment our bank to the next step
        self.curr_idx += 1;

        // If we have arrived at the end, return the charges
        if self.curr_idx >= self.charges.len() {
            return self.charges;
        } else {
            // Or continue recursively
            return self.fill_bank(&buffer_to_pick_from[where_i_found_it + 1..]);
        }
    }
}

pub fn solve() -> (u64, u64) {
    return BatteryBank::from_input(input())
        .iter()
        .map(|bank| {
            return (solve_part1(bank), solve_part2(bank));
        })
        // Count like this, because sum() does not work on tuples :(
        .fold((0, 0), |acc, next| {
            return (acc.0 + next.0, acc.1 + next.1);
        });
}

struct BatteryBank {
    battery_charges: Vec<u8>,
}

impl BatteryBank {
    fn from_input(input: &str) -> Vec<BatteryBank> {
        return input
            .lines()
            .map(|line| BatteryBank {
                battery_charges: line
                    .chars()
                    .map(|char| char.to_string().parse().unwrap())
                    .collect(),
            })
            .collect();
    }
}

#[cfg(test)]
fn input() -> &'static str {
    return "987654321111111\n811111111111119\n234234234234278\n818181911112111";
}

#[cfg(not(test))]
fn input() -> &'static str {
    return include_str!("./day03-input.txt");
}

#[test]
fn test() {
    assert_eq!(solve(), (357, 3121910778619));
}

pub fn get_solution() -> SolutionPair {
    let solved = solve();
    return (Solution::from(solved.0), Solution::from(solved.1));
}
