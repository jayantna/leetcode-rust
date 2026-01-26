use std::fs;

pub struct Advent_Of_Code;
impl Advent_Of_Code {
    pub fn can_fork_lift(paper_rolls: Vec<Vec<char>>) -> usize {
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
        let mut neighbours: Vec<Vec<char>> = Vec::new();
        for i in 0..paper_rolls.len() {
            for j in 0..paper_rolls[i].len() {
                let check: Vec<_> = directions
                    .iter()
                    .map(|x| {
                        i.checked_add_signed(x.0)
                            .and_then(|row_idx| paper_rolls.get(row_idx)) // Try to get row
                            .and_then(|row| j.checked_add_signed(x.1).and_then(|col| row.get(col))) // Try to get col
                            .copied()
                            .unwrap_or('.')
                    })
                    .collect();
                neighbours.push(check);
            }
        }
        // println!("{:?}", flatmap);
        neighbours.iter().flat_map(|x| x).filter(|&&x| x=='@').collect::<Vec<&char>>().len()
    }

    pub fn run() {
        let file = fs::read_to_string("./src/aoc/_4_aoc.txt").unwrap();
        let mut paper_rolls: Vec<_> = file
            .split('\n')
            .map(|x| x.chars().collect::<Vec<char>>())
            .collect();
        println!("{:?}", paper_rolls);
        let len = Advent_Of_Code::can_fork_lift(paper_rolls);
        println!("{:?}", len);
    }
}
