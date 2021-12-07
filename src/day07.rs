use crate::day::*;

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
        let line = io::BufReader::new(input).lines().next().ok_or(AocError)??;
        let positions = line.split(",").map(|s| s.parse().unwrap())
            .collect::<Vec<_>>();
        let costs = (0..=*positions.iter().max().ok_or(AocError)?)
            .map(|tgt| positions.iter().map(|&p| Output::abs(p - tgt)).sum());
        Ok(costs.min().ok_or(AocError)?)
    }

    fn part2_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let line = io::BufReader::new(input).lines().next().ok_or(AocError)??;
        let positions = line.split(",").map(|s| s.parse().unwrap())
            .collect::<Vec<_>>();
        let costs = (0..=*positions.iter().max().ok_or(AocError)?)
            .map(|tgt| positions.iter()
                .map(|&p| { let n = Output::abs(p - tgt); n * (1 + n) / 2 })
                .sum());
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