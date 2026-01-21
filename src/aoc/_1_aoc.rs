use std::ops::{Add, Sub};
use std::{fs, usize};

// Operator overloading of usize into u99 ring buffer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CirculatedU99(usize, usize);

impl CirculatedU99 {
    // Constructor that ensures the initial value is within 0-99
    fn new(value: usize) -> Self {
        CirculatedU99(value % 100, 0)
    }

    // Accessor for the inner value
    fn value(&self) -> usize {
        self.0
    }

    fn zero_trip(&self) -> usize {
        self.1
    }
}

// Implementation of Addition with circular logic
impl Add<usize> for CirculatedU99 {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        // zero trip to count number of times it crossed zero
        let zero_trip: usize = self.zero_trip() + (self.0 + rhs) / 100;
        // Circular addition: (current + change) % 100
        CirculatedU99((self.0 + rhs) % 100, zero_trip)
    }
}

// Implementation of Subtraction with circular logic
impl Sub<usize> for CirculatedU99 {
    type Output = Self;

    fn sub(self, rhs: usize) -> Self::Output {
        // Handle underflow by adding the modulus before taking the remainder
        let new_val = if self.0 >= (rhs % 100) {
            self.0 - (rhs % 100)
        } else {
            (self.0 + 100) - (rhs % 100)
        };
        let zero_trip = if self.0 <= (rhs % 100) && self.0 > 0 {
            self.zero_trip() + (rhs / 100) + 1
        } else {
            self.zero_trip() + (rhs / 100)
        };
        CirculatedU99(new_val, zero_trip)
    }
}
pub struct Advent_Of_Code;
impl Advent_Of_Code {
    pub fn get_zeros(chars: Vec<(char, usize)>) -> usize {
        let mut dial: CirculatedU99 = CirculatedU99(50, 0);
        let mut count = 0;
        for i in chars {
            if i.0 == 'R' {
                dial = dial + i.1
            } else {
                dial = dial - i.1
            }
            if dial.value() == 0 {
                count += 1;
            }
        }
        println!("{:?} {:?}", count, dial.zero_trip());
        0
    }

    pub fn run() {
        let mut rotations = String::new();
        match fs::read_to_string("./src/aoc/_1_aoc.txt") {
            Ok(x) => rotations = x,
            Err((e)) => println!("Read error {e}"),
        }
        let chars: Vec<(_, _)> = rotations
            .split('\n')
            .map(|x| (x.chars().nth(0).unwrap(), x[1..].parse::<usize>().unwrap()))
            .collect();
        let ans = Advent_Of_Code::get_zeros(chars);
        println!("{:?}", ans);
    }
}
