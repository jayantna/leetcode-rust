use std::{collections::HashSet, fs};

use itertools::Itertools;

/// Solution for Advent of Code 2025 Day 5.
pub struct Advent_Of_Code;

impl Advent_Of_Code {
    pub fn run() {
        // Read the input file
        let file = fs::read_to_string("./src/aoc/_5_aoc.txt").unwrap();
        let lines: Vec<_> = file.lines().collect();

        // Parse Ranges: The first section of the file containing "start-end" strings.
        // We take lines until we encounter an empty line separator.
        let mut ranges: Vec<_> = lines
            .iter()
            .take_while(|&&x| x != "")
            .map(|x| {
                let r = x.split('-').collect::<Vec<&str>>();
                (
                    r[0].parse::<usize>().unwrap(),
                    r[1].parse::<usize>().unwrap(),
                )
            })
            .collect();

        // Parse Ingredients: The second section of the file.
        // We iterate in reverse to get the bottom block up to the empty line separator.
        let ingredients: Vec<_> = lines
            .iter()
            .rev()
            .take_while(|&&x| x != "")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        // --- PART 1 ---
        // Count how many specific ingredients fall into at least one defined range.
        let mut fresh_count = 0;
        for id in ingredients {
            // Check if the ID is covered by any of the ranges
            let is_fresh = ranges.iter().any(|(start, end)| id >= *start && id <= *end);
            if is_fresh {
                fresh_count += 1;
            }
        }
        println!("Fresh count from ingredients {:?}", fresh_count);

        // --- PART 2 ---
        // Calculate the total number of unique integers covered by all ranges.
        // This requires merging overlapping ranges to avoid double counting.

        // 1. Sort ranges by their start value to allow for linear merging.
        ranges.sort();
        let mut total_fresh_count = 0;

        // 2. Merge overlapping ranges.
        let new_ranges = ranges.into_iter().fold(
            Vec::<(usize, usize)>::new(),
            |mut acc, (next_start, next_end)| {
                if let Some(last) = acc.last_mut() {
                    // If the current range overlaps with or is adjacent to the last range
                    // (Note: The code checks `last.1 >= next_start`. Since inputs are integers,
                    // strict adjacency (last.1 + 1 == next_start) might also be considered "mergeable"
                    // in some contexts, but here it looks for strict overlap or touching via >=).
                    if last.1 >= next_start {
                        // Merge them by extending the end of the last range
                        last.1 = last.1.max(next_end)
                    } else {
                        // No overlap, push as a new distinct range
                        acc.push((next_start, next_end));
                    }
                } else {
                    // Initialize with the first range
                    acc.push((next_start, next_end));
                }
                acc
            },
        );

        println!("{:?}", new_ranges);

        // 3. Calculate the total count of numbers covered by the merged ranges.
        let count = new_ranges
            .iter()
            .fold(0, |mut acc, (range_start, range_end)| {
                // Determine the length of the range (inclusive)
                acc += (*range_start..=*range_end).count();
                acc
            });

        println!("Total fresh count {:?}", count);
    }
}
