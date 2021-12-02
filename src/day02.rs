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

impl Day02 {
    fn part1_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let lines = io::BufReader::new(input).lines();
        lazy_static! {
            static ref RE: Regex = Regex::new("(.+) (.+)").unwrap();
        }
        let (d, l) = lines.fold((0, 0), |(d, h), l| {
            let l = l.unwrap();
            let cap = RE.captures(&l).unwrap();
            let dir: &str = &cap[1];
            let n = cap[2].parse::<i64>().unwrap();
            (d + if dir == "down" { n } else if dir == "up" { -n } else { 0 }, h + if dir =="forward" { n } else { 0 })
        });
        Ok(d * l)
    }

    fn part2_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let lines = io::BufReader::new(input).lines();
        lazy_static! {
            static ref RE: Regex = Regex::new("(.+) (.+)").unwrap();
        }
        let (d, l, _) = lines.fold((0, 0, 0), |(d, h, a), l| {
            let l = l.unwrap();
            let cap = RE.captures(&l).unwrap();
            let dir: &str = &cap[1];
            let n = cap[2].parse::<i64>().unwrap();
            (d + if dir == "forward" { n * a } else { 0 }, h + if dir =="forward" { n } else { 0 }, a + if dir == "down" { n } else if dir == "up" { -n } else { 0 })
        });
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
forward 2", 150);
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
forward 2", 900);
    }
}