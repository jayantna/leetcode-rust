use std::{fs, usize};

/// Solution for Advent of Code 2025 Day 4: "Printing Department"
pub struct Advent_Of_Code;

impl Advent_Of_Code {
    /// Determines which paper rolls ('@') are accessible to forklifts and removes them.
    ///
    /// A paper roll is accessible if it has fewer than 4 neighbors that are also paper rolls.
    /// This function performs a single pass of simultaneous updates:
    /// 1. Scans the grid to identify all accessible rolls based on the current state.
    /// 2. Removes those rolls by changing them to '.' after the scan is complete.
    ///
    /// Returns the number of rolls removed in this pass.
    pub fn can_fork_lift(paper_rolls: &mut Vec<Vec<char>>) -> usize {
        // Define the 8 directions (neighbors) to check: horizontal, vertical, and diagonal
        let directions = [
            (0, -1),
            (0, 1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        let mut accessible_count = 0;
        // Vector to store coordinates of rolls to be removed this round
        let mut elements_to_remove: Vec<(usize, usize)> = Vec::new();

        // Iterate through every cell in the grid
        for i in 0..paper_rolls.len() {
            for j in 0..paper_rolls[i].len() {
                // Check if the current cell contains a paper roll
                if paper_rolls[i][j] == '@' {
                    // Count how many neighbors are also paper rolls
                    let neighbors_count = directions
                        .iter()
                        .filter(|x| {
                            i.checked_add_signed(x.0)
                                .and_then(|row_idx| paper_rolls.get(row_idx))
                                .and_then(|row| {
                                    j.checked_add_signed(x.1).and_then(|col| row.get(col))
                                })
                                .copied()
                                .unwrap_or('.') // Treat out-of-bounds as empty ('.')
                                == '@'
                        })
                        .count();

                    // If a roll has fewer than 4 neighbors, it is accessible
                    if neighbors_count < 4 {
                        elements_to_remove.push((i, j));
                        accessible_count += 1;
                    }
                }
            }
        }

        // Apply the removals simultaneously after the scan
        for (i, j) in elements_to_remove {
            paper_rolls[i][j] = '.';
        }

        accessible_count
    }

    pub fn run() {
        let file = fs::read_to_string("./src/aoc/_4_aoc.txt").unwrap();
        // Parse the input file into a 2D grid of characters
        let mut paper_rolls: Vec<_> = file
            .split('\n')
            .map(|x| x.chars().collect::<Vec<char>>())
            .collect();

        let mut len = 0;
        // Continuously remove accessible rolls until no more can be removed
        loop {
            let thislen = Advent_Of_Code::can_fork_lift(&mut paper_rolls);
            len += thislen;
            println!("{:?}", len);

            // Break the loop if no rolls were removed in the current pass
            if thislen == 0 {
                break;
            }
        }
        println!("{:?}", len);
    }
}
