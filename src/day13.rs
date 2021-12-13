use crate::day::*;
use regex::Regex;
use std::collections::HashSet;

pub struct Day13 {}

type Output = usize;

impl Day for Day13 {
    fn tag(&self) -> &str { "13" }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{}", self.part2_impl(&mut *input()).unwrap());
    }
}

impl Day13 {
    fn part1_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let mut lines = io::BufReader::new(input).lines();
        let grid = lines.by_ref()
            .take_while(|line| line.as_ref().map_or(false, |line| !line.is_empty()))
            .map(|line| line.map_err(|e| e.into()).and_then(Self::parse_dot))
            .map(|r| r.unwrap()).collect::<Vec<_>>();
        let fold_spec = Self::parse_fold(lines.next().ok_or(AocError)??)?;
        Ok(Self::fold(&grid, fold_spec).len())
    }

    fn part2_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<String> {
        let mut lines = io::BufReader::new(input).lines();
        let grid = lines.by_ref()
            .take_while(|line| line.as_ref().map_or(false, |line| !line.is_empty()))
            .map(|line| line.map_err(|e| e.into()).and_then(Self::parse_dot))
            .map(|r| r.unwrap()).collect::<Vec<_>>();
        let grid = lines.fold(grid, |grid, line| {
            let fold_spec = Self::parse_fold(line.unwrap()).unwrap();
            Self::fold(&grid, fold_spec)
        });
        Ok(Self::visualise(&grid))
    }

    fn visualise(grid: &Vec<(usize, usize)>) -> String {
        let x_size = grid.iter().map(|cell| cell.0).max().unwrap() + 1;
        let y_size = grid.iter().map(|cell| cell.0).max().unwrap() + 1;
        let mut out = vec![vec![' '; x_size]; y_size];
        for (x, y) in grid {
            out[*y][*x] = '#';
        }
        out.into_iter().map(|row| row.into_iter().collect::<String>()).join("\n")
    }

    fn fold(grid: &Vec<(usize, usize)>, spec: (bool, usize)) -> Vec<(usize, usize)> {
        grid.iter().map(|(x, y)| {
            let (x, y) = (x.clone(), y.clone());
            match spec {
                (true, fold) => (if x > fold { fold - (x - fold) } else { x }, y),
                (false, fold) => (x, if y > fold { fold - (y - fold) } else { y })
            }
        }).collect::<HashSet<_>>().into_iter().collect()
    }

    fn parse_dot(s: String) -> BoxResult<(usize, usize)> {
        lazy_static! {
            static ref RE: Regex = Regex::new("(\\d+),(\\d+)").unwrap();
        }
        let cap = RE.captures(&s).ok_or(AocError)?;
        Ok((cap[1].parse()?, cap[2].parse()?))
    }

    fn parse_fold(s: String) -> BoxResult<(bool, usize)> {
        lazy_static! {
            static ref RE: Regex
                = Regex::new("fold along ([xy])=(\\d+)").unwrap();
        }
        let cap = RE.captures(&s).ok_or(AocError)?;
        Ok((&cap[1] == "x", cap[2].parse()?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day13 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1("6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5", 17);
    }

    fn test2(s: &str, f: &str) {
        assert_eq!(Day13 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f.to_string()));
    }

    #[test]
    fn part2() {
        test2("6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5", "#####
#   #
#   #
#   #
#####");
    }
}