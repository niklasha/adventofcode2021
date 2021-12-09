use closure::closure;
use crate::day::*;
use crate::day::io::Read;

pub struct Day09 {}

type Output = i64;

impl Day for Day09 {
    fn tag(&self) -> &str { "09" }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

impl Day09 {
    fn part1_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let (heights, xs, ys) = Self::parse(input);
        let low_points = (0..ys).flat_map(|y|
            (0..xs)
                .filter(
                    closure!(
                        move y, ref heights, |&x| Self::is_low(&heights, x, y)))
                .map(closure!(move y, ref heights, |x| heights[y][x])));
        Ok(low_points.map(|height| height + 1).sum())
    }

    fn part2_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let (heights, xs, ys) = Self::parse(input);
        // p2b maps pos to basin, bs is the sizes of basins, p represents a
        // position and is equal to x + y * y-size.
        // just iterate and create new basins whenever a position that does not
        // have a basin to the west or north.  Otherwise just mark it to belong
        // to that basin and increase its size.
        let mut p2b = vec![None; xs * ys];
        let mut bs: Vec<Output> = vec![];
        for y in 0..ys {
            for x in 0..xs {
                let basin
                    = if heights[y][x] == 9 { None }
                    else if y == 0 {
                        if x == 0 || p2b[x - 1 + y * xs] == None {
                            bs.resize(bs.len() + 1, 0);
                            Some(bs.len() - 1)
                        } else {
                            p2b[x - 1 + y * xs]
                        }
                    } else if x == 0 || p2b[x - 1 + y * xs] == None {
                        if p2b[x + (y - 1) * xs] == None {
                            bs.resize(bs.len() + 1, 0);
                            Some(bs.len() - 1)
                        } else {
                            p2b[x + (y - 1) * xs]
                        }
                    } else if p2b[x + (y - 1) * xs] == None
                        || p2b[x + (y - 1) * xs] == p2b[x - 1 + y * xs] {
                        p2b[x - 1 + y * xs]
                    } else {
                        // Merge basins
                        let west = p2b[x - 1 + y * xs];
                        let north = p2b[x + (y - 1) * xs];
                        for b in p2b.iter_mut() {
                            if *b == west { *b = north }
                        };
                        bs[north.unwrap()]
                            = bs[north.unwrap()] + bs[west.unwrap()];
                        bs[west.unwrap()] = 0;
                        north
                    };
                if let Some(b) = basin {
                    bs[b] = bs[b] + 1;
                    p2b[x + y * xs] = basin;
                }
            }
        }
        bs.sort();
        bs.reverse();
        Ok(bs[0] * bs[1] * bs[2])
    }

    fn parse(input: &mut dyn Read) -> (Vec<Vec<Output>>, usize, usize) {
        let heights = io::BufReader::new(input).lines()
            .map(|l| l.unwrap().chars().map(|c| c as Output - '0' as Output)
                .collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let xs = heights[0].len();
        let ys = heights.len();
        (heights, xs, ys)
    }

    fn is_low(heights: &Vec<Vec<Output>>, x: usize, y: usize) -> bool {
        (if x == 0 { x } else { x - 1 }
            ..=if x == heights[0].len() - 1 { x } else { x + 1 })
            .all(|nx|
                (if y == 0 { y } else { y - 1 }
                    ..=if y == heights.len() - 1 { y } else { y + 1 })
                    .filter(|&ny| (x != nx) != (y != ny))
                    .all(|ny| heights[y][x] < heights[ny][nx]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day09 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1(
            "2199943210
3987894921
9856789892
8767896789
9899965678",
            15);
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day09 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2(
            "2199943210
3987894921
9856789892
8767896789
9899965678",
            1134);
    }
}