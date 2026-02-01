use std::{collections::HashSet, fs};

pub struct Advent_Of_Code;
impl Advent_Of_Code {
    pub fn run() {
        // Read the puzzle input
        let file: String = fs::read_to_string("./src/aoc/_7_aoc.txt").unwrap();
        let lines: Vec<_> = file.lines().collect();

        // Part 1: Simulate beam activation and splitting
        // We use a HashSet to track which column horizontal positions are currently "active".
        let mut beam_set: HashSet<usize> =
            HashSet::from([{ lines[0].chars().position(|x| x == 'S').unwrap() }]);
        let mut count = 0;

        for line in lines.iter().copied() {
            // As we sweep through lines, if an active beam column encounters a '^' splitter,
            // it branches into the two adjacent columns and deactivates the current one.
            line.chars().enumerate().for_each(|(idx, x)| {
                if x == '^' && beam_set.contains(&idx) {
                    beam_set.insert(idx - 1); // Branch left
                    beam_set.insert(idx + 1); // Branch right
                    beam_set.remove(&idx); // Deactivate current
                    count += 1;
                }
            });
        }
        println!("Part 1 Count: {:?}", count);

    }
}
