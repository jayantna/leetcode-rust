use std::fs;
use std::ops::{Add, Mul};

use itertools::Itertools;
pub struct Advent_Of_Code;

impl Advent_Of_Code {
    pub fn run() {
        // Advent_Of_Code::part_1();
        Advent_Of_Code::part_2();
    }
    fn part_2() -> usize {
        // Read the input file and split into lines
        let file = fs::read_to_string("./src/aoc/_6_aoc.txt").unwrap();
        let lines: Vec<_> = file.lines().collect();
        // Identify "delimiters" - columns that are entirely composed of spaces across all lines
        let delimiters: Vec<usize> = (0..lines[0].chars().count())
            .filter(|&i| lines.iter().all(|line| line.chars().nth(i) == Some(' ')))
            .collect();
        // Create ranges representing chunks (columns) of numbers and their operations.
        // These ranges are bounded by the delimiters identified above.
        let solve_ranges: Vec<(usize, usize)> = std::iter::once(0)
            .chain(delimiters.iter().map(|&x| x + 1))
            .zip(
                delimiters
                    .iter()
                    .cloned()
                    .chain(std::iter::once(lines[0].chars().count())),
            )
            .collect();
        let mut total_sum = 0;
        // Process each range to extract numbers and apply the operator
        for (start, end) in solve_ranges {
            // The operator (+ or *) is on the last line within this specific range
            let operator = lines.iter().last().unwrap()[start..end]
                .trim()
                .parse::<char>()
                .unwrap();
            // Each number is formed by joining characters vertically from each column in the range,
            // taking all rows except the last one (which contains the operator).
            let numbers: Vec<_> = (start..end)
                .map(|x| {
                    let num = lines
                        .iter()
                        .take(lines.len() - 1)
                        .map(|lines| lines.chars().nth(x).unwrap())
                        .join("")
                        .trim()
                        .parse::<usize>()
                        .unwrap();
                    num
                })
                .collect();
            println!("{:?}", numbers);
            // Apply either sum or product based on the group's operator
            let result = match operator {
                '+' => numbers.iter().sum::<usize>(),
                '*' => numbers.iter().product::<usize>(),
                _ => 0,
            };
            total_sum += result;
        }
        println!("{:?}", total_sum);
        total_sum
    }

    fn part_1() {
        // Read the input file
        let file = fs::read_to_string("./src/aoc/_6_aoc.txt").unwrap();

        // Filter out empty lines to handle potential trailing newlines
        let lines: Vec<&str> = file.lines().filter(|l| !l.trim().is_empty()).collect();

        // The last line contains the operations (+, *)
        let ops_line = lines.last().unwrap();
        let ops: Vec<char> = ops_line
            .split_whitespace()
            .map(|s| s.chars().next().unwrap())
            .collect();

        // All preceding lines contain the numbers
        let number_lines = &lines[..lines.len() - 1];
        println!("{:?}", number_lines);
        // Parse the number grid. Each line is split by whitespace.
        let grid: Vec<Vec<u64>> = number_lines
            .iter()
            .map(|line| {
                line.split_whitespace()
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect()
            })
            .collect();

        //  Iterate on number grid from column to row
        let num_cols = ops.len();
        let mut total_sum = 0;

        // Iterate through each column first
        for col_idx in 0..num_cols {
            // Collect numbers for this column from all rows
            let mut col_numbers = Vec::new();
            for row in &grid {
                col_numbers.push(row[col_idx]);
            }

            // Determine the operation for this column
            let op = ops[col_idx];

            // Calculate result for the column based on the operation
            let col_result = match op {
                '+' => col_numbers.iter().sum::<u64>(),
                '*' => col_numbers.iter().product::<u64>(),
                _ => {
                    panic!("Unknown operation: {}", op);
                }
            };

            // Add the column result to the total
            total_sum += col_result;
        }

        println!("Result: {}", total_sum);
    }
}
