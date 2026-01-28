use std::fs;
use std::ops::{Add, Mul};
pub struct Advent_Of_Code;

impl Advent_Of_Code {
    pub fn run() {
        // Read the input file
        let file = fs::read_to_string("./src/aoc/_6_aoc.txt").unwrap();

        // Filter out empty lines to handle potential trailing newlines
        let lines: Vec<&str> = file.lines().filter(|l| !l.trim().is_empty()).collect();

        if lines.is_empty() {
            println!("Input file is empty.");
            return;
        }

        // The last line contains the operations (+, *)
        let ops_line = lines.last().unwrap();
        let ops: Vec<char> = ops_line
            .split_whitespace()
            .map(|s| s.chars().next().unwrap())
            .collect();

        // All preceding lines contain the numbers
        let number_lines = &lines[..lines.len() - 1];

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
