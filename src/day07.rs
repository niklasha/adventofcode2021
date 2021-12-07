use crate::day::*;
use crate::day::io::Read;
use crate::day::error::Error;

pub struct Day07 {}

type Output = i32;

impl Day for Day07 {
    fn tag(&self) -> &str { "07" }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

impl Day07 {
    fn part1_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<Output> {
        Self::minimal__fuel(
            input, |pos: Output, tgt: Output| Output::abs(pos -tgt))
    }

    fn part2_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<Output> {
        Self::minimal__fuel(
            input,
            |pos, tgt| { let n = Output::abs((pos - tgt)); n * (n + 1) / 2 })
    }

    fn minimal__fuel(input: &mut dyn Read, f: fn(Output, Output) -> Output)
        -> Result<i32, Box<dyn Error>> {
        let line = io::BufReader::new(input).lines().next().ok_or(AocError)??;
        let positions = line.split(",").map(|s| s.parse().unwrap())
            .collect::<Vec<_>>();
        let costs = (0..=*positions.iter().max().ok_or(AocError)?)
            .map(|tgt| positions.iter().map(|&p| f(p, tgt)).sum());
        Ok(costs.min().ok_or(AocError)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day07 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1("16,1,2,0,4,2,7,1,2,14", 37);
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day07 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2("16,1,2,0,4,2,7,1,2,14", 168);
    }
}