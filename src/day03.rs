use itertools::Itertools;
use itertools::FoldWhile::*;
use std::num::ParseIntError;
use crate::day::*;

pub struct Day03 {}

type Output = i64;

impl Day for Day03 {
    fn tag(&self) -> &str { "03" }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

impl Day03 {
    fn part1_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let dr = Utils::byte_matrix(input)?;
        let cs = dr[0].len();
        let (c0, c1) = dr.iter().fold(
            (vec![0 as Output; cs], vec![0 as Output; cs]), |(c0, c1), r| {
                let c0: Vec<Output> = c0.iter().zip(r.iter()).map(|(c, v)|
                    if *v == b'0' { *c + 1 } else { *c })
                    .collect::<Vec<_>>();
                let c1: Vec<Output> = c1.iter().zip(r.iter()).map(|(c, v)|
                    if *v == b'1' { *c + 1 } else { *c })
                    .collect::<Vec<_>>();
                (c0, c1)
            });
        let g = Self::bin_to_dec(
            &c0.iter().zip(c1.iter())
                .map(|(c0, c1)| if *c0 > *c1 { b'0' } else { b'1' }).collect())
            ?;
        Ok(g * Self::complement(g, cs))
    }

    fn part2_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let dr = Utils::byte_matrix(input)?;
        let cs = dr[0].len();
        let ogr = Self::compute_rating(&dr, cs, Self::most_common)?;
        let co2sr = Self::compute_rating(&dr, cs, Self::least_common)?;
        Ok(ogr * co2sr)
    }

    fn bin_to_dec(v: &Vec<u8>) -> Result<i64, ParseIntError> {
        i64::from_str_radix(
            &v.into_iter().map(|b| *b as char).collect::<String>(), 2)
    }

    fn complement(n: i64, l: usize) -> i64 {
        !n & ((1 << l) - 1)
    }

    fn most_common(v: &Vec<u8>) -> u8 {
        if v.iter().filter(|&c|
            *c == b'0').count() > v.len() / 2 { b'0' } else { b'1' }
    }

    fn least_common(v: &Vec<u8>) -> u8 {
        if v.iter().filter(|&c|
            *c == b'0').count() <= v.len() / 2 { b'0' } else { b'1' }
    }

    fn compute_rating(dr: &Vec<Vec<u8>>, cs: usize, f: fn(&Vec<u8>) -> u8) -> Result<i64, ParseIntError> {
        Self::bin_to_dec(
            &(0..cs).fold_while(dr.clone(), |candidates, i| {
                let mcb = f(&candidates.iter().map(|r| r[i]).collect());
                let n = candidates.into_iter().filter(|r| r[i] == mcb).collect::<Vec<_>>();
                if n.len() == 1 { Done(n) } else { Continue(n.clone()) }
            }).into_inner()[0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day03 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1("00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010",
              198);
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day03 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2("00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010",
              230);
    }
}