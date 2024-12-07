use crate::problem::{DailySolution, Problem};

pub struct Day4;

const data: &'static str = include_str!("../../data/day4.txt");

const directions: [(i32, i32); 8] = [
    (0, 1),
    (1, 0),
    (0, -1),
    (-1, 0),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
]; // all directions
const directions2: [(i32, i32); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)]; // corner dirctions only

impl Problem for Day4 {
    fn part1(&self, d: &str) -> Option<String> {
        let grid = Day4::parse_file(d);
        let mut result = 0;
        let r = grid.len();
        let c = grid[0].len();

        for i in 0..r {
            for j in 0..c {
                // start all traversals from X
                if grid[i][j] == 'X' {
                    // check all directions for the Next letter M
                    // then in the same direction for M -> A -> S
                    for (dx, dy) in directions.iter() {
                        let mut x = i as i32 + dx;
                        let mut y = j as i32 + dy;
                        // count ensure the letters are found in the correct order
                        let mut count = 0;
                        while x >= 0 && x < r as i32 && y >= 0 && y < c as i32 {
                            if grid[x as usize][y as usize] == 'M' && count == 0 {
                                count += 1;
                            } else if grid[x as usize][y as usize] == 'A' && count == 1 {
                                count += 1;
                            } else if grid[x as usize][y as usize] == 'S' && count == 2 {
                                count += 1;
                                result += 1;
                            } else {
                                break;
                            }
                            // to continue in the same direction
                            x += dx;
                            y += dy;
                        }
                    }
                }
            }
        }

        Some(result.to_string())
    }

    fn part2(&self, d: &str) -> Option<String> {
        let grid = Day4::parse_file(d);
        let mut result = 0;
        let r = grid.len();
        let c = grid[0].len();

        for i in 0..r {
            for j in 0..c {
                if grid[i][j] == 'A' {
                    let mut corners = vec![];
                    for (dx, dy) in directions2.iter() {
                        let x = i as i32 + dx;
                        let y = j as i32 + dy;

                        while x >= 0 && x < r as i32 && y >= 0 && y < c as i32 {
                            if grid[x as usize][y as usize] == 'M'
                                || grid[x as usize][y as usize] == 'S'
                            {
                                corners.push(grid[x as usize][y as usize]);
                            }

                            break;
                        }
                    }
                    if corners.len() == 4 {
                        // let [top_left, top_right, bottom_left, bottom_right] = corners.as_slice();
                        if corners[0] == corners[1]
                            && (corners[0] == 'M' || corners[0] == 'S')
                            && corners[2] == corners[3]
                            && (corners[2] == 'M' || corners[2] == 'S')
                            && corners[0] != corners[2]
                        {
                            result += 1;
                        } else if corners[0] == corners[2]
                            && (corners[0] == 'M' || corners[0] == 'S')
                            && corners[1] == corners[3]
                            && (corners[1] == 'M' || corners[1] == 'S')
                            && corners[0] != corners[1]
                        {
                            result += 1;
                        }
                    }
                }
            }
        }
        Some(result.to_string())
    }

    fn answer(&self) -> DailySolution {
        DailySolution::new(4, self.part1(data), self.part2(data))
    }
}

impl Day4 {
    fn parse_file(d: &str) -> Vec<Vec<char>> {
        let mut grid = Vec::new();
        for line in d.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(c);
            }
            grid.push(row);
        }
        grid
    }
}

#[cfg(test)]
mod tests {

    use crate::problem::Problem;

    use super::Day4;

    #[test]
    fn test_part1() {
        let data = "XMAS\nSAMX\nMMMM\nMMMM";
        println!("{:?}", Day4::parse_file(data));

        assert_eq!(Day4::part1(&Day4, data), Some("2".to_string()));
    }

    #[test]
    fn test_part2() {
        let data = "SXSMXSMXMSXM\nXAXXAXXAXXAX\nMXMMXSSXSSXM";
        assert_eq!(Day4::part2(&Day4, data), Some("4".to_string()));
    }

    #[test]
    fn test_a_count() {
        let mut count = 0;
        Day4::parse_file(super::data).iter().for_each(|row| {
            count += row.iter().filter(|&c| *c == 'A').count();
        });

        println!("{:?}", count);
    }
}
