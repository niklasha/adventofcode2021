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

// XXX Could drop Undetermined and instead model the state as an
// XXX Either<Output, ShotResult> for a better type safety.
#[derive(Clone, Copy, Debug)]
enum ShotState {
    Undetermined(Output),
    Hit(Output),
    Miss
}

impl ShotState {
    fn height(self) -> Option<Output> {
        match self {
            ShotState::Undetermined(h) | ShotState::Hit(h) => Some(h),
            _ => None
        }
    }
}

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

    fn get_lowest_vx_that_reaches(target: &Area) -> BoxResult<Output> {
        (1..).map(|vx| (vx, vx * (vx + 1) / 2)).find(|(_, x)| x >= &target.0)
            .map(|(vx, _)| vx).ok_or(AocError.into())
    }

    fn find_max_height(target: Area) -> BoxResult<Output> {
        let low_vx = Self::get_lowest_vx_that_reaches(&target)?;
        // XXX Assume target is lower that our starting point
        let heights = (0..=-target.2).flat_map(|vy|
            (low_vx..=target.1).map(move |vx|
                Self::shot(&target, vx, vy, Some(0)).height()))
            .flatten();
        Ok(heights.max().ok_or(AocError)?)
    }

    fn count(target: Area) -> BoxResult<Output> {
        let low_vx = Self::get_lowest_vx_that_reaches(&target)?;
        // XXX Assume target is lower that our starting point
        let heights = (target.2..=-target.2).flat_map(|vy|
            (low_vx..=target.1).map(move |vx|
                Self::shot(&target, vx, vy, None).height()))
            .flatten();
        Ok(heights.count() as Output)
    }

    fn shot(target: &Area, vx: Output, vy: Output, limit: Option<Output>)
        -> ShotState {
        use ShotState::*;
        let (tx0, tx1, ty0, ty1) = *target;
        (0..).fold_while(
            (0, 0, vx, vy, Undetermined(0)),
            |(x, y, vx, vy, state), _| {
                let (nx, ny) = (x + vx, y + vy);
                let early_termination = limit.map_or(
                    false,
                    |limit|
                        vy <= 0 && state.height().map_or(true, |h| h < limit));
                if nx >= tx0 && ny <= ty1 || ny < ty0 || nx > tx1
                    || early_termination {
                    Done(
                        (x, y, vx, vy,
                         if nx >= tx0 && nx <= tx1 && ny >= ty0 && ny <= ty1
                            { state.height().map_or(state, |h| Hit(h)) }
                         else { Miss }))
                } else {
                    Continue(
                        (nx, ny, if vx > 0 { vx - 1 } else { 0 }, vy - 1,
                         Undetermined(
                             state.height().map_or(ny, |h| cmp::max(ny, h)))))
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