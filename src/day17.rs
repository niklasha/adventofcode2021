use crate::day::*;
use regex::Regex;
use itertools::FoldWhile::{Done, Continue};
use std::cmp;

pub struct Day17 {}

type Output = i64;

impl Day for Day17 {
    fn tag(&self) -> &str { "17" }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

type Area = (Output, Output, Output, Output);

impl Day17 {
    fn parse(s: String) -> BoxResult<Area> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                "target area: x=(\\d+)..(\\d+), y=(-?\\d+)..(-?\\d+)")
                .unwrap();
        }
        let cap = RE.captures(&s).ok_or(AocError)?;
        Ok((cap[1].parse()?, cap[2].parse()?, cap[3].parse()?, cap[4].parse()?))
    }

    fn part1_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let target = Self::parse(
            io::BufReader::new(input).lines().next()
                .ok_or(AocError)??)?;
        Self::find_max_height(target)
    }

    fn part2_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let target = Self::parse(
            io::BufReader::new(input).lines().next()
                .ok_or(AocError)??)?;
        Self::count(target)
    }

    fn find_max_height(target: Area) -> BoxResult<Output> {
        let heights = (0..).flat_map(|vy| {
            (1..=target.1).map(move |vx| Self::shot(&target, vx, vy, Some(0)))
        }).take(100000).flatten(); // XXX what's the correct ending condition?
        Ok(heights.max().ok_or(AocError)?)
    }

    fn count(target: Area) -> BoxResult<Output> {
        let heights = (target.2..).flat_map(|vy| {
            (1..=target.1).map(move |vx| Self::shot(&target, vx, vy, None))
        }).take(100000).flatten(); // XXX what's the correct ending condition?
        Ok(heights.count() as Output)
    }

    fn shot(target: &Area, vx: Output, vy: Output, limit: Option<Output>) -> Option<Output> {
        let (tx0, tx1, ty0, ty1) = *target;
        (0..).fold_while(
            (0, 0, vx, vy, Some(0)),
            |(x, y, vx, vy, h), _| {
                let (nx, ny) = (x + vx, y + vy);
                if nx >= tx0 && ny <= ty1 || ny < ty0 || nx > tx1
                    || limit.map_or(
                        false,
                        |limit| vy <= 0 && h.map_or(true, |h| h < limit)) {
                    Done(
                        (x, y, vx, vy,
                         if nx >= tx0 && nx <= tx1 && ny >= ty0 && ny <= ty1
                            { h }
                         else { None }))
                } else {
                    Continue(
                        (nx, ny, if vx > 0 { vx - 1 } else { 0 }, vy - 1,
                         Some(h.map_or(ny, |h| cmp::max(ny, h)))))
                }
            }).into_inner().4
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day17 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1("target area: x=20..30, y=-10..-5", 45);
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day17 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2("target area: x=20..30, y=-10..-5", 112);
    }
}