use crate::day::*;

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
    fn part1_impl(self: &Self, input: &mut dyn io::Read, days: u32) -> BoxResult<Output> {
        let l = io::BufReader::new(input).lines().next().ok_or(AocError)??;
        let mut v = l.split(",").map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        (0..days).for_each(|i| {
            (0..v.len()).for_each(|i|
                if v[i] == 0 { v[i] = 6; v.push(8) } else { v[i] = v[i] - 1 });
        });
        Ok(v.len())
    }

    fn part2_impl(self: &Self, input: &mut dyn io::Read, days: usize) -> BoxResult<Output> {
        let l = io::BufReader::new(input).lines().next().ok_or(AocError)??;
        let mut v = vec![0; 9];
        l.split(",").map(|s| s.parse::<usize>().unwrap()).for_each(|d| {
            v[d] = v[d] + 1;
        });
        (0..days).for_each(|_| {
            let n = v.remove(0);
            v[6] = v[6] + n;
            v.push(n);
        });
        Ok(v.iter().sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, days: u32, f: Output) {
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