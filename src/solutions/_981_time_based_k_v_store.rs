use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn run() {
        let mut time_map = TimeMap::new();
        time_map.set("foo".to_string(), "bar".to_string(), 1);
        println!(
            "get('foo', 1): '{}' (expected: 'bar')",
            time_map.get("foo".to_string(), 1)
        );
        println!(
            "get('foo', 3): '{}' (expected: 'bar')",
            time_map.get("foo".to_string(), 3)
        );

        time_map.set("foo".to_string(), "bar2".to_string(), 4);
        println!(
            "get('foo', 4): '{}' (expected: 'bar2')",
            time_map.get("foo".to_string(), 4)
        );
        println!(
            "get('foo', 5): '{}' (expected: 'bar2')",
            time_map.get("foo".to_string(), 5)
        );
    }
}

struct TimeMap {
    // <key: [(timestamp, value)]>
    time_map: HashMap<String, Vec<(i32, String)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {
    fn new() -> Self {
        TimeMap {
            time_map: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.time_map
            .entry(key)
            .or_insert(Vec::new())
            .push((timestamp, value));
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        if let Some(values) = self.time_map.get(&key) {
            let mut start = 0;
            let mut end = values.len();

            // Binary search for the upper bound (first elem > timestamp)
            while start < end {
                let mid = start + (end - start) / 2;
                if values[mid].0 <= timestamp {
                    start = mid + 1;
                } else {
                    end = mid;
                }
            }

            // valid elements are at indices < start.
            // value at start is > timestamp.
            // so we want values[start - 1] if start > 0.
            if start == 0 {
                return String::from("");
            }
            return values[start - 1].1.clone();
        }
        String::from("")
    }
}
