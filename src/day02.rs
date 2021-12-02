use itertools::Itertools;
use regex::Regex;
use crate::day::*;

pub struct Day02 {}

type Output = i64;

impl Day for Day02 {
    fn tag(&self) -> &str { "02" }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

pub enum Command {
    Forward(Output),
    Down(Output),
    Up(Output),
}
use Command::*;

impl Day02 {
    fn parse(s: &str) -> BoxResult<Command> {
        lazy_static! {
            static ref RE: Regex = Regex::new("(.+) (.+)").unwrap();
        }
        let cap = RE.captures(s).ok_or(AocError)?;
        let n = cap[2].parse::<i64>()?;
        match &cap[1] {
            "forward" => Ok(Forward(n)),
            "down" => Ok(Down(n)),
            "up" => Ok(Up(n)),
            _ => Err(Box::new(AocError))
        }
    }

    fn part1_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let mut lines = io::BufReader::new(input).lines();
        let (d, l) = lines.fold_ok((0, 0), |(d, h), l| {
            match Self::parse(&l).unwrap() {
                Forward(n) => (d, h + n),
                Down(n) => (d + n, h),
                Up(n) => (d - n, h)
            }
        })?;
        Ok(d * l)
    }

    fn part2_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let mut lines = io::BufReader::new(input).lines();
        let (d, l, _) = lines.fold_ok((0, 0, 0), |(d, h, a), l| {
            match Self::parse(&l).unwrap() {
                Forward(n) => (d + n * a, h + n, a),
                Down(n) => (d, h, a + n),
                Up(n) => (d, h, a - n)
            }
        })?;
        Ok(d * l)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day02 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1("forward 5
down 5
forward 8
up 3
down 8
forward 2",
              150);
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day02 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2("forward 5
down 5
forward 8
up 3
down 8
forward 2",
              900);
    }
}