use crate::day::*;
use itertools::Itertools;

pub struct Day01 {}

type Output = usize;

impl Day for Day01 {
    fn tag(&self) -> &str { "01" }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

impl Day01 {
    fn part1_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<Output> {
        Ok(Utils::numbers(input).map(Result::unwrap).tuple_windows()
            .filter(|(a, b)| a < b).count())
    }

    fn part2_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<Output> {
        Ok(Utils::numbers(input).map(Result::unwrap).tuple_windows()
            .map(|(a, b, c)| a + b + c).tuple_windows().filter(|(a, b)| a < b)
            .count())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day01 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1("199
200
208
210
200
207
240
269
260
263", 7);
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day01 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2("199
200
208
210
200
207
240
269
260
263", 5);
    }
}