use crate::problem::{DailySolution, Problem};
use regex::Regex;
pub struct Day3;
const data: &'static str = include_str!("../../data/day3.txt");

impl Problem for Day3 {
    fn part1(&self, d: &str) -> Option<String> {
        let valid_muls = Day3::parse_file(d);
        let mut result: u128 = 0;
        let _: Vec<_> = valid_muls.iter().filter(|&s| s != "don't()" && s != "do()").map(|mul| {
            println!("{:?}", mul);
            let (left,right) = mul.split_once(',').unwrap();
            let left_val = left.split_once('(').unwrap().1.parse::<u32>().unwrap();
            let right_val = right.split_once(')').unwrap().0.parse::<u32>().unwrap();

            result += left_val as u128 * right_val as u128;
        }).collect();

        Some(result.to_string())
    }

    fn part2(&self, d: &str) -> Option<String> {
        let valid_muls = Day3::parse_file(d);
        let mut result: u128 = 0;
        let mut off = false;

        for mul in valid_muls.iter() {
            println!("{:?}", mul);
            println!("{:?}", off);
            if mul == "do()" {
                off = false;
                continue;
            }
            else if mul == "don't()" {
                off = true;
                continue;
            }
            else if off {
                continue;
            }
            
            
            let (left,right) = mul.split_once(',').unwrap();
            let left_val = left.split_once('(').unwrap().1.parse::<u32>().unwrap();
            let right_val = right.split_once(')').unwrap().0.parse::<u32>().unwrap();

            result += left_val as u128 * right_val as u128;
        }

        Some(result.to_string())
    }
    fn answer(&self) -> crate::problem::DailySolution {
        DailySolution::new(3, self.part1(data), self.part2(data))
    }
}

impl Day3 {
    // return all valid instances of mul(X,Y) where X and Y are integers
    fn parse_file(d: &str) -> Vec<String> {
        let re = Regex::new(r"do(n't)?(\(\))|mul\((\d+),(\d+)\)").unwrap();

        re.find_iter(d).map(|m| m.as_str().to_string()).collect()

        // idea 1
        // add another regex for do / don't
        // get ordering of each command via byte offset in the str to construct a properly ordered list of commands

        // idea 2
        // do a preprocessing of the string to remove all characters from the index of a `don't()` to a `do()`
        
    }
}

#[cfg(test)]
mod tests {
    use crate::problem::Problem;

    use super::Day3;

    #[test]
    fn test_parse_file() {

        let data = "soemthingmul(2,3)jajamul+(2,4)kdon't()lomul(2*,9)ppoqwmul(123,89)";
        let result = Day3::parse_file(data);
        println!("{:?}", result);  
    }

    #[test]
    fn test_part1() {

        // Day3::part2(&Day3);
        // assert_eq!(Day3::part1(&Day3), Some("0".to_string()));
    }
}
