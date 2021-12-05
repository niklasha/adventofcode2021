use num::signum;
use regex::Regex;
use std::collections::HashMap;
use std::ops::RangeInclusive;
use crate::day::*;

pub struct Day05 {}

type Output = usize;

impl Day for Day05 {
    fn tag(&self) -> &str { "05" }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

impl Day05 {
    fn parse(s: &str) -> BoxResult<((u32, u32), (u32, u32))> {
        lazy_static! {
            static ref RE: Regex = Regex::new("(\\d+),(\\d+) -> (\\d+),(\\d++)").unwrap();
        }
        let cap = RE.captures(s).ok_or(AocError)?;
        Ok(((cap[1].parse::<u32>()?, cap[2].parse::<u32>()?),
            (cap[3].parse::<u32>()?, cap[4].parse::<u32>()?)))
    }

    fn part1_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let mut m = HashMap::new();
        let n = io::BufReader::new(input).lines()
            .map(|l| Self::parse(&l.unwrap()).unwrap())
            .filter(|((x1, y1), (x2, y2))| x1 == x2 || y1 == y2)
            .fold(0, |n, ((x1, y1), (x2, y2))| {
                if x1 == x2 {
                    Self::range(y1, y2).fold(n, |n, y| {
                        let v: u32 = *m.get(&(x1, y)).unwrap_or(&0);
                        m.insert((x1, y), v + 1);
                        if v == 1 { n + 1 } else { n }
                    })
                } else {
                    Self::range(x1, x2).fold(n, |n, x| {
                        let v = *m.get(&(x, y1)).unwrap_or(&0);
                        m.insert((x, y1), v + 1);
                        if v == 1 { n + 1 } else { n }
                    })
                }
            });
        Ok(n)
    }

    fn range(a: u32, b: u32) -> RangeInclusive<u32> {
        if a > b { b..=a } else { a..=b }
    }

    fn part2_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let mut m = HashMap::new();
        let n = io::BufReader::new(input).lines()
            .map(|l| Self::parse(&l.unwrap()).unwrap())
            .fold(0, |n, ((x1, y1), (x2, y2))| {
                if x1 == x2 {
                    Self::range(y1, y2).fold(n, |n, y| {
                        let v: u32 = *m.get(&(x1, y)).unwrap_or(&0);
                        m.insert((x1, y), v + 1);
                        if v == 1 { n + 1 } else { n }
                    })
                } else if y1 == y2 {
                    Self::range(x1, x2).fold(n, |n, x| {
                        let v = *m.get(&(x, y1)).unwrap_or(&0);
                        m.insert((x, y1), v + 1);
                        if v == 1 { n + 1 } else { n }
                    })
                } else /*if i32::abs(x1 as i32 - x2 as i32) == i32::abs(y1 as i32 - y2 as i32)*/ {
                    let dx = signum(x2 as i32 - x1 as i32);
                    let dy = signum(y2 as i32 - y1 as i32);
                    (0..=i32::abs(x2 as i32 - x1 as i32)).fold(n, |n, i| {
                        let x = (x1 as i32 + dx * i) as u32;
                        let y = (y1 as i32 + dy * i) as u32;
                        let v = *m.get(&(x, y)).unwrap_or(&0);
                        m.insert((x, y), v + 1);
                        if v == 1 { n + 1 } else { n }
                    })
//                } else {
//                    n
                }
            });
        Ok(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day05 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1("0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2",
              5);
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day05 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2("0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2",
              12);
    }
}