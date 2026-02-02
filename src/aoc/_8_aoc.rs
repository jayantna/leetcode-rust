use std::fs;

pub struct Advent_Of_Code;
impl Advent_Of_Code {
    pub fn run() {
        // Read the puzzle input
        let file: String = fs::read_to_string("./src/aoc/_8_aoc.txt").unwrap();
        let lines: Vec<_> = file.lines().collect();
        lines.iter().for_each(|&x| println!("{:?}", {
            Advent_Of_Code::get_distance(("162,817,812"), x)
        }));
    }

    fn get_distance(a: &str, b: &str) -> f32 {
        let m: Vec<_> = a.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
        let n: Vec<_> = b.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
        let dist = (((m[0] - n[0]).pow(2) + (m[1] - n[1]).pow(2) + (m[2] - n[2]).pow(2)) as f32).sqrt();
        return dist;
    }
}
