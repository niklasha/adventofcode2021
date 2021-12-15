use crate::day::*;
use itertools::FoldWhile::{Done, Continue};
use std::iter;

pub struct Day15 {}

type Output = i32;

impl Day for Day15 {
    fn tag(&self) -> &str { "15" }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

impl Day15 {
    fn part1_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<Output> {
        self.process(input, |grid| grid)
    }

    fn part2_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<Output> {
        self.process(input, Self::expand_part2)
    }

    fn process(self: &Self, input: &mut dyn io::Read, f: fn(Vec<Vec<(Output, Option<Output>)>>) -> Vec<Vec<(Output, Option<Output>)>>) -> BoxResult<Output> {
        let mut grid = f(
            Utils::byte_matrix(input)?.into_iter().map(|row|
                row.into_iter().map(|cell|
                    ((cell - b'0') as Output, None as Option<Output>))
                    .collect::<Vec<_>>())
                .collect::<Vec<_>>());
        let exit = (grid[0].len() - 1, grid.len() - 1);
        let _ = (0..).fold_while(
            vec![(vec![(0, 0)], 0)],
            |paths, _| {
                let paths = paths.into_iter()
                    .flat_map(|path| Self::walk(&mut grid, &path))
                    .collect::<Vec<_>>();
                let best = grid[exit.1][exit.0].1;
                let done = best.is_some()
                    && paths.iter().all(|(_, risk)| *risk >= best.unwrap());
                if done { Done(paths) } else { Continue(paths) }
            })
            .into_inner();
        grid[exit.1][exit.0].1.ok_or(AocError.into())
    }

    fn expand_part2(grid: Vec<Vec<(Output, Option<Output>)>>)
        -> Vec<Vec<(Output, Option<Output>)>> {
        let (sx0, sy0) = (grid[0].len(), grid.len());
        (0..sy0 * 5).map(|y|
            (0..sx0 * 5).map(|x|
                ((grid[y % sy0][x % sx0].0 + (x / sx0 + y / sy0) as Output - 1)
                     % 9 + 1,
                 None))
                .collect::<Vec<_>>())
            .collect::<Vec<_>>()
    }

    fn walk(grid: &mut Vec<Vec<(Output, Option<Output>)>>,
            path: &(Vec<(usize, usize)>, Output))
        -> Vec<(Vec<(usize, usize)>, Output)> {
        let (ux, uy) = path.0[path.0.len() - 1];
        if grid[uy][ux].1.map_or(false, |b| b < path.1) { return vec![]; }
        let (x, y) = (ux as isize, uy as isize);
        let (usx, usy) = (grid[0].len(), grid.len());
        let (sx, sy) = (usx as isize, usy as isize);
        [(-1, 0), (0, -1), (1, 0), (0, 1)].iter().flat_map(|(dx, dy)|
            if x + dx < 0 || y + dy < 0 || x + dx >= sx || y + dy >= sy
                || path.0.contains(&((x + dx) as usize, (y + dy) as usize))
                { None }
            else { Some(((x + dx) as usize, (y + dy) as usize)) })
            .flat_map(|(x, y)| {
                let (local_risk, risk_so_far) = grid[y][x];
                let risk = path.1 + local_risk;
                if risk_so_far.is_none() || risk < risk_so_far.unwrap() {
                    grid[y][x].1 = Some(risk);
                    Some(
                        (path.0.iter().cloned().chain(iter::once((x, y)))
                             .collect(),
                         risk))
                } else { None }
            }).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day15 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1("1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581", 40);
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day15 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2("1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581", 315);
    }
}