#![allow(dead_code, unreachable_code, unused_imports)]
use crate::{Solution, SolutionPair};
use std::{collections::HashSet, hash::Hash, ops::DerefMut, u64, usize};

/**
 * Part 1:
 * You get a grid with either empty spaces "." or rolls of paper "@".
 * You need to figure out which rolls of paper can be moved by a forklift.
 * A forklift can move the roll of paper, when there are fewer than 4 rolls of paper in the spaces next to it.
 * (Horizontal, vertical and diagonal neighbours all count)
 *
 * Part 2:
 * Instead of requiring us to check how many paper rolls can be removed right now.
 * We now need to check how many paper rolls can be removed at all.
 * Basically there are layouts, where not all paper rolls can be removed,
 * because they can form clumps where every roll has more than 4 neighbours and nothing can be removed.
 *
 * The challange is to do an iterative approach, and check how many rolls can be removed total.
 */

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord)]
struct RollPosition {
    x: i32,
    y: i32,
}
impl RollPosition {
    fn get_possible_neighbour_positions(&self) -> [RollPosition; 8] {
        let (x, y) = (self.x, self.y);
        return [
            RollPosition { x: x + 1, y: y + 1 },
            RollPosition { x: x + 1, y: y + 0 },
            RollPosition { x: x + 1, y: y - 1 },
            RollPosition { x: x + 0, y: y + 1 },
            RollPosition { x: x + 0, y: y - 1 },
            RollPosition { x: x - 1, y: y + 1 },
            RollPosition { x: x - 1, y: y + 0 },
            RollPosition { x: x - 1, y: y - 1 },
        ];
    }
}

pub fn solve() -> (u64, u64) {
    let mut rolls: HashSet<RollPosition> = input()
        .lines()
        .enumerate()
        .map(|(row_idx, line)| {
            return line
                .chars()
                .enumerate()
                .filter(|(_, char)| *char == '@')
                .map(move |(col_idx, _)| RollPosition {
                    x: col_idx as i32,
                    y: row_idx as i32,
                });
        })
        .flatten()
        .collect();

    let part1_solution = remove_1_iteration_of_paper_rolls(&mut rolls);
    let mut part2_solution = part1_solution; // and everything thats added in the loop

    loop {
        let rolls_removed = remove_1_iteration_of_paper_rolls(&mut rolls);

        if rolls_removed == 0 {
            break;
        }

        part2_solution += rolls_removed;
    }

    return (part1_solution, part2_solution);
}

/// Checks how many neighbours exist for a given position
fn check_for_paper_roll_neighbours(
    all_rolls: &HashSet<RollPosition>,
    roll_to_check_for: &RollPosition,
) -> u8 {
    return roll_to_check_for
        .get_possible_neighbour_positions()
        .iter()
        .filter(|neighbour| all_rolls.contains(neighbour))
        .count() as u8;
}

fn remove_1_iteration_of_paper_rolls(all_rolls: &mut HashSet<RollPosition>) -> u64 {
    let mut removed_count = 0u64;

    *all_rolls = all_rolls
        .iter()
        .filter(|pos| {
            let neighbour_count = check_for_paper_roll_neighbours(all_rolls, pos);

            if neighbour_count < 4 {
                removed_count += 1;
                return false;
            } else {
                return true;
            }
        })
        .map(|roll| *roll)
        .collect();

    return removed_count;
}

#[cfg(test)]
fn input() -> &'static str {
    return "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";
}

#[cfg(not(test))]
fn input() -> &'static str {
    return include_str!("./day04-input.txt");
}

#[test]
fn test() {
    assert_eq!(solve(), (13, 43));
}

pub fn get_solution() -> SolutionPair {
    let solved = solve();
    return (Solution::from(solved.0), Solution::from(solved.1));
}
