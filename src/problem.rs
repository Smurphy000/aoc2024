use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DailySolution {
    // day the problem opened
    day: u8,
    // solution to part 1 for the day
    part1: Option<String>,
    // solution to part 2 for the day
    part2: Option<String>,
}

impl DailySolution {
    pub fn new(day: u8, part1: Option<String>, part2: Option<String>) -> Self {
        Self { day, part1, part2 }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Solutions {
    // Contains all daily solutions
    pub days: Vec<DailySolution>,
}
pub trait Problem {
    fn part1(&self) -> Option<String> {
        None
    }
    fn part2(&self) -> Option<String> {
        None
    }
    fn answer(&self) -> DailySolution {
        DailySolution {
            day: 0,
            part1: None,
            part2: None,
        }
    }
}
