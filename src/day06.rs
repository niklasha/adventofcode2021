use crate::day::*;
use crate::day::io::Read;
use crate::day::error::Error;

pub struct Day06 {}

type Output = usize;

impl Day for Day06 {
    fn tag(&self) -> &str { "06" }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input(), 80));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input(), 256));
    }
}

impl Day06 {
    fn part1_impl(self: &Self, input: &mut dyn io::Read, days: usize)
        -> BoxResult<Output> {
        Self::process(input, days)
    }

    fn part2_impl(self: &Self, input: &mut dyn io::Read, days: usize)
        -> BoxResult<Output> {
        Self::process(input, days)
    }

    fn process(input: &mut dyn Read, days: usize) -> Result<usize, Box<dyn Error>> {
        let line = io::BufReader::new(input).lines().next().ok_or(AocError)??;
        let due_times = line.split(",").map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        let init = (0..9).map(|i| due_times.iter().filter(|&e| *e == i).count())
            .collect::<Vec<_>>();
        let due_time_counts = (0..days).fold(init, |mut counts, _| {
            let spawn_count = counts.remove(0);
            counts[6] = counts[6] + spawn_count;
            counts.push(spawn_count);
            counts
        });
        Ok(due_time_counts.iter().sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, days: usize, f: Output) {
        assert_eq!(Day06 {}.part1_impl(&mut s.as_bytes(), days).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1("3,4,3,1,2", 18, 26);
        test1("3,4,3,1,2", 80, 5934);
    }

    fn test2(s: &str, days: usize, f: Output) {
        assert_eq!(Day06 {}.part2_impl(&mut s.as_bytes(), days).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2("3,4,3,1,2", 256, 26984457539);
    }
}