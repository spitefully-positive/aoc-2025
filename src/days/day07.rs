#![allow(dead_code, unreachable_code, unused_imports)]
use crate::{Solution, SolutionPair};
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    ops::DerefMut,
    rc::Rc,
    u64,
};

type ColumnIndex = usize;
type Count = u64;
type SplitterPosition = (usize, usize);

pub fn solve() -> (u64, u64) {
    let mut input = input().lines();

    let starting_point = input
        .next()
        .expect("Could not iterate over input at all")
        .chars()
        .position(|char| char == 'S')
        .expect("Could not find \"S\" in first line.");

    let (beam_timelines, splitters_we_met) = input.into_iter().enumerate().fold(
        (
            HashMap::<ColumnIndex, Count>::from([(starting_point, 1u64)]),
            HashSet::<SplitterPosition>::new(),
        ),
        |(beams, mut splitters), (line_idx, line)| {
            let splitters_hit_on_this_line = line
                .chars()
                .enumerate()
                .filter_map(|(col_idx, char)| match char {
                    '^' => Some((line_idx, col_idx)),
                    _ => None,
                })
                .collect::<HashSet<_>>();

            // Only ever keep track of beams on the current line, or else memory might explode
            let mut new_beams = HashMap::<ColumnIndex, Count>::new();

            for (beam_idx, beam_count) in beams {
                if splitters_hit_on_this_line.contains(&(line_idx, beam_idx)) {
                    splitters.insert((line_idx, beam_idx));
                    new_beams
                        .entry(beam_idx + 1)
                        .and_modify(|val| *val += beam_count)
                        .or_insert(beam_count);
                    new_beams
                        .entry(beam_idx - 1)
                        .and_modify(|val| *val += beam_count)
                        .or_insert(beam_count);
                } else {
                    new_beams
                        .entry(beam_idx)
                        .and_modify(|val| *val += beam_count)
                        .or_insert(beam_count);
                }
            }

            return (new_beams, splitters);
        },
    );

    let sol1: u64 = splitters_we_met.len() as u64;
    let sol2: u64 = beam_timelines.into_values().sum();

    return (sol1, sol2);
}

#[cfg(test)]
fn input() -> &'static str {
    return ".......S.......\n...............\n.......^.......\n...............\n......^.^......\n...............\n.....^.^.^.....\n...............\n....^.^...^....\n...............\n...^.^...^.^...\n...............\n..^...^.....^..\n...............\n.^.^.^.^.^...^.\n...............";
}

#[cfg(not(test))]
fn input() -> &'static str {
    return include_str!("./day07-input.txt");
}

#[test]
fn test() {
    assert_eq!(solve(), (21, 40));
}

pub fn get_solution() -> SolutionPair {
    let solved = solve();
    return (Solution::from(solved.0), Solution::from(solved.1));
}
