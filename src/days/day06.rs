#![allow(dead_code, unreachable_code, unused_imports)]
use crate::{Solution, SolutionPair};
use std::{collections::HashMap, u64};

/**
 * Part 1:
 *
 * Part 2:
 *
 */

pub fn solve() -> (u64, u64) {
    let mut input = input().lines();

    let mut operation_list = input
        .next_back()
        .unwrap()
        .chars()
        .filter(|char| *char != ' ')
        .map(|char| {
            return match char {
                '+' => Operators::Add,
                '*' => Operators::Multiply,
                char => panic!("We got another char than we expected. Char was \"{char}\""),
            };
        })
        .map(|operator| {
            return Operation {
                values: vec![],
                operator,
            };
        })
        .collect::<Vec<_>>();

    input.into_iter().for_each(|line| {
        for (chunk_idx, chunk) in line.chars().collect::<Vec<_>>().chunks(4).enumerate() {
            if chunk.len() > 4 || chunk.len() < 3 {
                panic!(
                    "Unexpected Chunk size. Chunk \"{}\"",
                    chunk.iter().collect::<String>()
                )
            }

            if let Some(operation) = operation_list.get_mut(chunk_idx) {
                operation.add_value(chunk[0..=2].iter().collect::<String>());
            } else {
                continue;
            }
        }
    });

    let (sol1, sol2) =
        operation_list
            .into_iter()
            .fold((0u64, 0u64), |(part1_acc, part2_acc), operation| {
                return (
                    part1_acc + operation.calculate_part_1(),
                    part2_acc + operation.calculate_part_2(),
                );
            });

    return (sol1, sol2);
}

struct Operation {
    operator: Operators,
    values: Vec<String>,
}
impl Operation {
    fn add_value(&mut self, value: String) {
        self.values.push(value);
    }

    fn calculate_part_1(&self) -> u64 {
        return self
            .values
            .iter()
            .map(|val| val.trim().parse::<u64>().unwrap())
            .reduce(|acc, val| {
                return match self.operator {
                    Operators::Add => acc + val,
                    Operators::Multiply => acc * val,
                };
            })
            .unwrap_or(0);
    }

    fn calculate_part_2(&self) -> u64 {
        let mut weird_values: HashMap<usize, u64> = HashMap::new();

        todo!(
            "My Problem is, that I need to determine the exponent depending on how many digits the number will have in the end"
        );
        /*
        * [src/days/day06.rs:114:9] &self.values = [
            "123", // This happens because everything in this line gets taken 10.pow(2), even though the power is dependent on how many other digits are in that column
            " 45",
            "  6",
        ]
        [src/days/day06.rs:114:9] weird_values.values() = [
            240,
            100,
            356,
        ]

        should be:
        24
        1
        356


        */
        self.values
            .iter()
            .rev()
            .enumerate()
            .flat_map(|(exponent, val)| {
                return val
                    .chars()
                    .filter_map(|(exponent, val)| val.to_digit(10).map(|num| num as u64))
                    .rev()
                    .enumerate()
                    .filter_map(move |(vertical_number_id, char)| {
                        if let Some(num) = char.to_digit(10) {
                            return Some((vertical_number_id, exponent, num as u64));
                        } else {
                            return None;
                        }
                    });
            })
            .for_each(|(vertical_number_id, exponent, num)| {
                let previous_value = *weird_values.get(&vertical_number_id).unwrap_or(&0);

                _ = weird_values.insert(
                    vertical_number_id,
                    previous_value + (num * 10u64.pow(exponent as u32)),
                );
            });

        dbg!(&self.values, weird_values.values(), &self.operator);

        return weird_values
            .into_values()
            .reduce(|acc, current_value| {
                return match self.operator {
                    Operators::Add => acc + current_value,
                    Operators::Multiply => acc * current_value,
                };
            })
            .unwrap_or(0);
    }
}

#[derive(Debug)]
enum Operators {
    Add,
    Multiply,
}

#[cfg(test)]
fn input() -> &'static str {
    return "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";
}

#[cfg(not(test))]
fn input() -> &'static str {
    return include_str!("./day06-input.txt");
}

#[test]
fn test() {
    assert_eq!(solve(), (4277556, 3263827));
}

pub fn get_solution() -> SolutionPair {
    let solved = solve();
    return (Solution::from(solved.0), Solution::from(solved.1));
}
