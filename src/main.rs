mod problem;
use problem::{Problem, Solutions};
mod problems;

use problems::{day1::Day1, day2::Day2, day3::Day3, day4::Day4};
fn main() {
    // TODO load from file so as to not recompute previously completed days
    let solutions = Solutions {
        days: vec![
            Day1::answer(&Day1),
            Day2::answer(&Day2),
            Day3::answer(&Day3),
            Day4::answer(&Day4),
        ],
    };
    println!("{:#?}", solutions);
}

// Want to add CLI to run or override answers for particular day
