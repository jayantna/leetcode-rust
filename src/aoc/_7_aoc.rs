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

        // Part 2: Calculate specific manifold values (beam quantities)
        // Instead of just tracking presence, we track the 'quantity' of beams at each column.
        let mut manifolds = vec![0; lines[0].len()];
        // Initialize the starting position 'S' with 1 unit.
        manifolds[lines[0].chars().position(|x| x == 'S').unwrap()] = 1;

        for line in lines.iter().copied() {
            line.chars().enumerate().for_each(|(idx, x)| {
                if x == '^' {
                    // When hitting a splitter, the current column's value is distributed
                    // to its left and right neighbors, and the current index is cleared.
                    let current_val = manifolds[idx];
                    manifolds[idx - 1] += current_val;
                    manifolds[idx + 1] += current_val;
                    manifolds[idx] = 0;
                }
            });
            // Summary for the current row
            println!("Manifolds: {:?}", manifolds);
            let sum = manifolds.iter().sum::<usize>();
            println!("Row Sum: {:?}", sum);
        }
    }
}
