#![allow(dead_code, unreachable_code, unused_imports)]
use crate::{Solution, SolutionPair};
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    u64,
};

/**
 * Part 1:
 *
 * Part 2:
 *
 */

pub fn solve() -> (u64, u64) {
    let mut id_generator = IdGenerator::new();

    let mut circuits: Vec<Circuit> = vec![];

    let mut alone_boxes = input()
        .lines()
        .map(|line| {
            let numbers = line
                .split(',')
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<_>>();

            return JunctionBox {
                position: Position {
                    x: numbers[0],
                    y: numbers[1],
                    z: numbers[2],
                },
            };
        })
        .collect::<Vec<_>>();

    let test = circuits
        .iter_mut()
        .flat_map(|circuit| {
            return circuit
                .junction_boxes
                .iter()
                .map(move |junction| (Some(circuit), junction));
        })
        .chain(alone_boxes.iter().map(|junction| (None, junction)))
        .collect::<Vec<_>>();

    // .chain(alone_boxes.iter().map(| |))

    let sol1: u64 = 0;
    let sol2: u64 = 0;

    return (sol1, sol2);
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Circuit {
    id: u64,
    junction_boxes: HashSet<JunctionBox>,
}
impl Circuit {
    fn new(id_generator: &mut IdGenerator) -> Self {
        let id = id_generator.next();
        Circuit {
            id,
            junction_boxes: HashSet::new(),
        }
    }

    fn add_junction_box(&mut self, junction_box: JunctionBox) {
        self.junction_boxes.insert(junction_box);
    }

    fn merge_with(mut self, other: Circuit) {
        self.junction_boxes.extend(other.junction_boxes);
    }
}

impl Hash for Circuit {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct JunctionBox {
    position: Position,
}
impl JunctionBox {
    fn distance_to(&self, other: &JunctionBox) -> f64 {
        return self.position.distance_to(&other.position);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i64,
    y: i64,
    z: i64,
}

impl Position {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Position { x, y, z }
    }

    fn distance_to(&self, other: &Position) -> f64 {
        let diff_x = self.x - other.x;
        let diff_y = self.y - other.y;
        let diff_z = self.z - other.z;
        let diff_all = (diff_x.pow(2) + diff_y.pow(2) + diff_z.pow(2)) as f64;
        return diff_all.sqrt().abs(); // Distance should be positive for now
    }
}

struct IdGenerator {
    next_id: u64,
}
impl IdGenerator {
    fn new() -> Self {
        IdGenerator { next_id: 0 }
    }

    fn next(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        return id;
    }
}

#[cfg(test)]
fn input() -> &'static str {
    return "162,817,812\n57,618,57\n906,360,560\n592,479,940\n352,342,300\n466,668,158\n542,29,236\n431,825,988\n739,650,466\n52,470,668\n216,146,977\n819,987,18\n117,168,530\n805,96,715\n346,949,466\n970,615,88\n941,993,340\n862,61,35\n984,92,344\n425,690,689";
}

#[cfg(not(test))]
fn input() -> &'static str {
    return include_str!("./day08-input.txt");
}

#[test]
fn test() {
    assert_eq!(solve(), (0, 0));
}

pub fn get_solution() -> SolutionPair {
    let solved = solve();
    return (Solution::from(solved.0), Solution::from(solved.1));
}
