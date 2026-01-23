use std::fs;

pub struct Advent_Of_Code;
impl Advent_Of_Code {
    pub fn max_jolt_part1(s: &str) -> usize {
        let nums: Vec<_> = s.chars().map(|x| x.to_digit(10).unwrap()).collect();
        let mut left_max = 0;
        let mut right_max = 0;
        for i in 0..nums.len() {
            if nums[i] > left_max && i != nums.len() - 1 {
                left_max = nums[i];
                right_max = nums[i + 1];
                continue;
            }
            if nums[i] > right_max {
                right_max = nums[i];
            }
        }
        println!("{:?}{:?}", left_max, right_max);
        format!("{:?}{:?}", left_max, right_max)
            .parse::<usize>()
            .unwrap()
    }

    pub fn max_jolt_part2(s: &str) -> usize {
        let nums: Vec<u32> = s.chars().map(|x| x.to_digit(10).unwrap()).collect();
        let mut result = String::new();
        let mut start_idx = 0;

        for i in 0..12 {
            // On each loop reduce the window size to check max on it
            // When there is no more posibility left to reduce window size,
            // then window size will become 1 and elements starts appending at the end to complete 12 length.
            let remaining_needed = 12 - i;

            let end_search_idx = nums.len() - remaining_needed;

            let mut max_val = 0;
            let mut max_idx = start_idx;

            // Create window to check max into.
            for idx in start_idx..=end_search_idx {
                if nums[idx] > max_val {
                    max_val = nums[idx];
                    max_idx = idx;
                }
                // If we find 9, we can stop immediately because no digit > 9
                if max_val == 9 {
                    break;
                }
            }

            // Append max at the end to form jolt
            result.push_str(&max_val.to_string());
            start_idx = max_idx + 1;
        }

        result.parse::<usize>().unwrap()
    }

    pub fn run() {
        let file = fs::read_to_string("./src/aoc/_3_aoc.txt").unwrap();
        let batteries: Vec<&str> = file.split('\n').collect();
        let mut total_jolts = 0;
        for i in batteries {
            println!("{:?}", i);
            total_jolts += Advent_Of_Code::max_jolt_part2(i);
        }
        println!("{:?}", total_jolts);
        // println!("{:?}", batteries);
    }
}
