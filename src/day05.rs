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

type Vent = ((i32, i32), (i32, i32));

impl Day05 {
    fn parse(s: &str) -> BoxResult<Vent> {
        lazy_static! {
            static ref RE: Regex
                = Regex::new("(\\d+),(\\d+) -> (\\d+),(\\d++)").unwrap();
        }
        let cap = RE.captures(s).ok_or(AocError)?;
        Ok(((cap[1].parse()?, cap[2].parse()?),
            (cap[3].parse()?, cap[4].parse()?)))
    }

    fn part1_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let vents: Vec<Vent> = io::BufReader::new(input).lines()
            .map(|l| Self::parse(&l.unwrap()).unwrap())
            .filter(|((x1, y1), (x2, y2))| x1 == x2 || y1 == y2).collect();
        Ok(Self::compute_overlap(&vents))
    }

    fn part2_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let vents: Vec<Vent> = io::BufReader::new(input).lines()
            .map(|l| Self::parse(&l.unwrap()).unwrap()).collect();
        Ok(Self::compute_overlap(&vents))
    }

    fn compute_overlap(vents: &Vec<Vent>) -> usize {
        let mut m = HashMap::new();
        vents.into_iter().fold(0, |n, &((x1, y1), (x2, y2))| {
            let c = i32::abs(if x1 == x2 { y2 - y1 } else { x2 -x1 });
            let dx = signum(x2 - x1);
            let dy = signum(y2 - y1);
            (0..=c).fold(n, |n, i| {
                let x = x1 + dx * i;
                let y = y1 + dy * i;
                let v = *m.get(&(x, y)).unwrap_or(&0);
                m.insert((x, y), v + 1);
                if v == 1 { n + 1 } else { n }
            })
        })
    }

    fn range(a: i32, b: i32) -> RangeInclusive<i32> {
        if a > b { b..=a } else { a..=b }
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