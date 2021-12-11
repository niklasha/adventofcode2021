use itertools::FoldWhile::{Done, Continue};
use crate::day::*;

pub struct Day11 {}

type Output = i64;

impl Day for Day11 {
    fn tag(&self) -> &str { "11" }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

impl Day11 {
    fn part1_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let map = Utils::byte_matrix(input)?.into_iter().map(|row|
            row.into_iter().map(|byte| byte as u32 - '0' as u32)
                .collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let dim = (map[0].len(), map.len());
        let (flashes, _) = (0..100).fold((0, map), |(mut flashes, mut map), _| {
            // First
            map.iter_mut().for_each(|row|
                (*row).iter_mut().for_each(|octopus| *octopus = *octopus + 1));
            // Then
            let energized = (0..dim.0).flat_map(|x| (0..dim.1)
                .map(move |y| (x, y)))
                .filter(|&(x, y)| map[y][x] > 9).collect::<Vec<_>>();
            energized.iter().for_each(|&(x, y)|
                flashes = flashes + Self::flash(&mut map, &dim, x, y));
            // Finally
            map.iter_mut().for_each(|row|
                (*row).iter_mut().filter(|octopus| **octopus > 9)
                .for_each(|octopus| *octopus = 0));
            (flashes, map)
        });
        Ok(flashes)
    }

    fn part2_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let map = Utils::byte_matrix(input)?.into_iter().map(|row|
            row.into_iter().map(|byte| byte as u32 - '0' as u32)
                .collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let dim = (map[0].len(), map.len());
        let size = (dim.0 * dim.1) as Output;
        let (_, _, all) = (1 as Output..).fold_while((0, map, 0), |(mut flashes, mut map, _), i| {
            // First
            map.iter_mut().for_each(|row|
                (*row).iter_mut().for_each(|octopus| *octopus = *octopus + 1));
            // Then
            let energized = (0..dim.0).flat_map(|x| (0..dim.1)
                .map(move |y| (x, y)))
                .filter(|&(x, y)| map[y][x] > 9).collect::<Vec<_>>();
            let new: Output
                = energized.iter()
                .map(|&(x, y)| Self::flash(&mut map, &dim, x, y)).sum();
            // Finally
            map.iter_mut().for_each(|row|
                (*row).iter_mut().filter(|octopus| **octopus > 9)
                    .for_each(|octopus| *octopus = 0));
            if (new == size) {
                Done((flashes + new, map, i))
            } else {
                Continue((flashes + new, map, i))
            }
        }).into_inner();
        Ok(all)
    }

    fn flash(map: &mut Vec<Vec<u32>>, dim: &(usize, usize), x: usize, y: usize) -> Output {
        let x_range = if x != 0 { x - 1 } else { x }
            ..=if x != dim.0 - 1 { x + 1 } else { x };
        let neighbours = x_range.flat_map(|nx| {
            let y_range = if y != 0 { y - 1 } else { y }
                ..=if y != dim.0 - 1 { y + 1 } else { y };
            y_range.filter(move |&ny| nx != x || ny != y).map(move |ny| (nx, ny))
        }).collect::<Vec<_>>();
        let energized = neighbours.into_iter().flat_map(|(x, y)| {
            map[y][x] = map[y][x] + 1;
            if map[y][x] == 10 {
                Some((x, y))
            } else { None }
        })
            .collect::<Vec<_>>();
        energized.into_iter().map(|(x, y)| Self::flash(map, dim, x, y))
            .sum::<Output>()
            + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day11 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1("5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526", 1656)
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day11 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2("5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526", 195);
    }
}