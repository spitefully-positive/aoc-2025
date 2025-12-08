use crate::{Solution, SolutionPair};
use std::u64;

/**
 * Part 1:
 * 0 to 99 (over flowing and underflowing)
 * Always starts at 50
 * Puzzle Input starts with L or R for left and right
 * R means number gets bigger, L smaller
 * Start at 11 + R7 = 18
 * Start at 11 + L7 = 4
 * Count how many times we land EXACTLY at 0
 * Moving over zero like from 99 to 1 with R2 will not count
 * The solution is a positive number, depending how often we move over zero
 *
 * Part 2:
 * Now count at any time we cross the number zero, not just when we exactly land on zero
 */

fn solve() -> (u64, u64) {
    let input = input();

    let mut times_at_zero: u64 = 0;
    let mut times_we_went_over_zero: u64 = 0;

    let input = parse_input(input);

    input.into_iter().fold(
        Lock::new(),
        /* initial lock position */
        |mut lock, operation| {
            let was_at_zero_before_turning = lock.is_at_zero();

            let overflows = lock.apply_operation(&operation);
            times_we_went_over_zero += overflows;

            if was_at_zero_before_turning == false && lock.is_at_zero() {
                times_at_zero += 1;
            }

            return lock;
        },
    );

    return (times_at_zero, times_at_zero + times_we_went_over_zero);
}

pub struct Lock {
    position: i64,
}

impl Lock {
    fn new() -> Lock {
        Lock { position: 50 }
    }

    fn is_at_zero(&self) -> bool {
        self.position == 0
    }

    /// Applies an operation to the lock and returns hof often the lock was overflown
    fn apply_operation(&mut self, operation: &LockOperation) -> u64 {
        let was_at_zero_before_operation = self.is_at_zero();

        // take away the complete turns early and count the overshoots
        let mut times_lock_was_overflown = (operation.distance / 100) as u64;
        let distance_left_to_turn = operation.distance % 100;

        self.position = match operation.direction {
            Direction::Left => self.position - distance_left_to_turn as i64,
            Direction::Right => self.position + distance_left_to_turn as i64,
        };

        if self.position.abs() == 100 {
            self.position = 0
        }

        // Reset if we are out of range
        if self.position > 100 {
            if was_at_zero_before_operation == false {
                times_lock_was_overflown += 1
            };
            self.position -= 100;
        } else if self.position < 0 {
            if was_at_zero_before_operation == false {
                times_lock_was_overflown += 1
            };
            self.position += 100;
        }

        return times_lock_was_overflown;
    }
}

enum Direction {
    Left,
    Right,
}

struct LockOperation {
    pub direction: Direction,
    pub distance: u16,
}

fn parse_input(input: &str) -> Vec<LockOperation> {
    return input
        .lines()
        .map(|line| {
            let line = line.trim();
            let (direction, distance) = line.split_at(1);
            let distance = distance.parse().unwrap();
            let direction = match direction {
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("Invalid direction"),
            };

            LockOperation {
                direction,
                distance,
            }
        })
        .collect::<Vec<LockOperation>>();
}

#[cfg(test)]
fn input() -> &'static str {
    return "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82\n";
}
#[cfg(not(test))]
fn input() -> &'static str {
    return include_str!("./day01-input.txt");
}

#[test]
fn test() {
    assert_eq!(solve(), (3, 6));
}

pub fn get_solution() -> SolutionPair {
    let solved = solve();
    return (Solution::from(solved.0), Solution::from(solved.1));
}
