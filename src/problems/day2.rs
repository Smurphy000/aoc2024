use crate::problem::{DailySolution, Problem};
use std::{
    fs::File,
    io::{self, BufRead},
};

pub struct Day2;

impl Problem for Day2 {
    fn part1(&self) -> Option<String> {
        let reports = Self::parse_file().unwrap();

        let result = reports
            .iter()
            .filter(|report| Self::dampen(0, report) == true)
            .count();

        Some(result.to_string())
    }
    fn part2(&self) -> Option<String> {
        let reports = Self::parse_file().unwrap();

        let result = reports
            .iter()
            .filter(|report| Self::dampen(1, report) == true)
            .count();

        Some(result.to_string())
    }
    fn answer(&self) -> DailySolution {
        DailySolution::new(2, self.part1(), self.part2())
    }
}

impl Day2 {
    const N: usize = 1000;
    const F: &str = "./data/day2.txt";

    fn parse_file() -> Result<Vec<Vec<u32>>, io::Error> {
        let f = File::open(Self::F)?;
        let mut reports = Vec::with_capacity(Self::N);
        for line in io::BufReader::new(f)
            .lines()
            .enumerate()
            .map(|(i, l)| l.unwrap_or_else(|_| panic!("Error parsing line {}", i)))
        {
            // Final line in file is empty
            if line == "" {
                continue;
            }
            let report = line
                .split(" ")
                .into_iter()
                .map(|v| v.parse().unwrap())
                .collect::<Vec<u32>>();
            reports.push(report);
        }
        Ok(reports)
    }

    fn analyze_report(report: &Vec<u32>) -> u32 {
        // determine if increasing or decreasing based off first two items
        // then ensure any increase or decrease is between 1 and 3
        let mut current = report[0];
        let mut current_direction: Option<bool> = None;
        let mut error_count: u32 = 0;
        for i in 1..report.len() {
            let diff: i32 = current as i32 - report[i] as i32;
            let increasing = if diff > 0 { true } else { false };
            let mut check_same = false;
            if let Some(same) = current_direction {
                if same == increasing {
                    check_same = true;
                }
            } else {
                current_direction = Some(increasing);
                check_same = true;
            }

            let range = if diff.abs() >= 1 && diff.abs() <= 3 {
                true
            } else {
                false
            };

            current = report[i];

            if !(check_same && range) {
                error_count += 1;
            }
        }
        error_count
    }

    //
    fn dampen(threshold: u8, report: &Vec<u32>) -> bool {
        let reported_errors = Day2::analyze_report(report);

        if reported_errors > threshold.into() {
            return false;
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::Day2;
    #[test]
    fn test_analyze() {
        let input = vec![0, 1, 2, 3];
        let result = Day2::analyze_report(&input);
        println!("{}", result);
    }
    #[test]

    fn test_dampen() {
        let input = vec![0, 1, 2, 3];
        let result = Day2::dampen(0, &input);
        println!("{}", result);
    }
    #[test]

    fn test_dampen_fails() {
        let input = vec![0, 1, 0, 1];
        let result = Day2::dampen(1, &input);
        println!("{}", result);
    }
}
