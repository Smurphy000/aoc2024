use crate::problem::{DailySolution, Problem};

pub struct Day3;

impl Problem for Day3 {
    fn part1(&self) -> Option<String> {
        None
    }

    fn part2(&self) -> Option<String> {
        None
    }
    fn answer(&self) -> crate::problem::DailySolution {
        DailySolution::new(3, self.part1(), self.part2())
    }
}

impl Day3 {
    fn parse_file() {}
}
