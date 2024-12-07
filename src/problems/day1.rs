use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

use crate::problem::{DailySolution, Problem};

pub struct Day1;

impl Problem for Day1 {
    fn part1(&self,d: &str) -> Option<String> {
        let (mut left, mut right) = Self::parse_file().unwrap();
        left.sort();
        right.sort();

        let result = left
            .iter()
            .enumerate()
            .map(|(i, x)| x.abs_diff(right[i]))
            .sum::<u128>();

        Some(result.to_string())
    }

    fn part2(&self, d: &str) -> Option<String> {
        let (left, right) = Day1::parse_file().unwrap();

        let mut mapping: HashMap<u128, u32> = HashMap::new();

        let _: i32 = left
            .iter()
            .map(|&x| {
                mapping.insert(x, 0);
                0
            })
            .sum();

        let _: i32 = right
            .iter()
            .map(|x| {
                (match mapping.get_mut(x) {
                    Some(x) => *x += 1,
                    None => {}
                });
                0
            })
            .sum();

        let result = mapping.iter().map(|(&k, &v)| k * v as u128).sum::<u128>();

        Some(result.to_string())
    }

    fn answer(&self) -> DailySolution {
        DailySolution::new(1, self.part1(""), self.part2(""))
    }
}

impl Day1 {
    const N: usize = 1000;
    const F: &str = "./data/day1.txt";
    fn parse_file() -> Result<(Vec<u128>, Vec<u128>), io::Error> {
        let f = File::open(Self::F)?;

        let mut left: Vec<u128> = Vec::with_capacity(Self::N);
        let mut right: Vec<u128> = Vec::with_capacity(Self::N);
        for line in io::BufReader::new(f)
            .lines()
            .enumerate()
            .map(|(i, l)| l.unwrap_or_else(|_| panic!("Error parsing line {}", i)))
        {
            // Final line in file is empty
            if line == "" {
                continue;
            }
            let row_split = line.split("   ").collect::<Vec<&str>>();
            left.push(row_split[0].parse().unwrap());
            right.push(row_split[1].parse().unwrap());
        }

        Ok((left, right))
    }
}
