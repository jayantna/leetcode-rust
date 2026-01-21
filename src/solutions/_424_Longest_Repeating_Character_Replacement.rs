use std::collections::{HashMap, HashSet};

pub struct Solution;
impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut counts = [0; 26]; // Frequency array for 'A'-'Z'
        let mut left = 0;
        let mut max_freq = 0;
        let mut max_len = 0;

        for right in 0..chars.len() {
            // Update frequency for the new character
            let idx = (chars[right] as usize) - ('A' as usize);
            counts[idx] += 1;

            // max_freq is the highest frequency of any single character seen in the window
            max_freq = max_freq.max(counts[idx]);

            // If (window_size - max_freq) > k, the window is invalid
            let window_len = (right - left + 1) as i32;
            if window_len - max_freq > k {
                // Shrink from the left
                let left_idx = (chars[left] as usize) - ('A' as usize);
                counts[left_idx] -= 1;
                left += 1;
            }

            // Record the largest valid window size
            max_len = max_len.max((right - left + 1) as i32);
        }

        max_len
    }
    pub fn run() {
        let s = String::from("AABABBA");
        let k = 1;
        let ans = Solution::character_replacement(s, k);
        println!("{:?}", ans);
    }
}
