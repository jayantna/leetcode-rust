use std::fs;

use itertools::Itertools;
pub struct Advent_Of_Code;
impl Advent_Of_Code {
    pub fn get_total_invalid(r: Vec<(usize, usize)>) -> usize {
        let mut invalid_ids = 0;
        for i in r {
            for j in (i.0..=i.1) {
                if !Advent_Of_Code::check_invalid(j) {
                    invalid_ids += j;
                }
            }
        }
        invalid_ids
    }

    fn check_invalid(num: usize) -> bool {
        let mut valid = true;
        let num_vec: Vec<_> = num
            .to_string()
            .chars()
            .map(|x| x.to_digit(10).unwrap())
            .collect();
        for factor in (1..num_vec.len()).filter(|x| num_vec.len() % x == 0) {   //numbers in which can be grouped
            let chunked: Vec<&[u32]> = num_vec.chunks(factor).collect();   // make group of thos e numbers
            let zipped: Vec<(&&[u32], &&[u32])> =
                chunked.iter().zip(chunked.iter().skip(1)).collect();   // zip adjacent numbers sequencially to prapre for comparision
            let same_subseq = zipped.iter().map(|(x, y)| x == y).all(|x| x == true);  // Comapre the zipped elements of they are equal
            if same_subseq == true {  // If any of them is equal, then break out of the loop and return invalid
                valid = false;
                break;
            }
        }
        valid
    }

    pub fn run() {
        let file_string = fs::read_to_string("./src/aoc/_2_aoc.txt").unwrap();
        let ranges: Vec<_> = file_string.split(',').collect();
        let range_tuples: Vec<(_, _)> = ranges
            .iter()
            .map(|&x| {
                let r = x.split('-').collect::<Vec<&str>>();
                return (
                    r[0].parse::<usize>().unwrap(),
                    r[1].parse::<usize>().unwrap(),
                );
            })
            .collect();
        let ans = Advent_Of_Code::get_total_invalid(range_tuples);
        println!("{:?}", ans);
    }
}
