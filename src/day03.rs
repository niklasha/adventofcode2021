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
    fn bin_to_dec(v: &Vec<u8>) -> i64 {
        i64::from_str_radix(&v.into_iter().map(|b| *b as char).collect::<String>(), 2).unwrap()
    }

    fn part1_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let dr = &Utils::byte_matrix(input).unwrap();
        let cs = dr[0].len();
        let (c0, c1) = dr.iter().fold((vec![0 as Output; cs], vec![0 as Output; cs]), |(c0, c1), r| {
            let c0: Vec<Output> = c0.iter().zip(r.iter()).map(|(c, v)|
                if *v == b'0' { *c + 1 } else { *c })
                .collect::<Vec<_>>();
            let c1: Vec<Output> = c1.iter().zip(r.iter()).map(|(c, v)|
                if *v == b'1' { *c + 1 } else { *c })
                .collect::<Vec<_>>();
            (c0, c1)
        });
        let g = i64::from_str_radix(
            &c0.iter().zip(c1.iter()).map(|(c0, c1)| if *c0 > *c1 { '0' } else { '1' }).into_iter().collect::<String>(), 2).unwrap();
        let e = i64::from_str_radix(
            &c0.iter().zip(c1.iter()).map(|(c0, c1)| if *c1 > *c0 { '0' } else { '1' }).into_iter().collect::<String>(), 2).unwrap();
        Ok(e * g)
    }

    fn most_common(v: &Vec<u8>) -> u8 {
        if v.iter().filter(|&c| *c == b'0').count() > v.len() / 2 { b'0' } else { b'1' }
    }

    fn least_common(v: &Vec<u8>) -> u8 {
        if v.iter().filter(|&c| *c == b'0').count() <= v.len() / 2 { b'0' } else { b'1' }
    }

    fn part2_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let dr = Utils::byte_matrix(input).unwrap();
        let cs = dr[0].len();
        let (_, ogr) = (0..cs).fold((dr.clone(), None), |(candidates, r), i| {
            let mcb = Self::most_common(&candidates.iter().map(|r| r[i]).collect());
            if r == None {
                let n = candidates.into_iter().filter(|r| r[i] == mcb).collect::<Vec<_>>();
                (n.clone(), if n.len() == 1 { Some(n[0].clone()) } else { None })
            } else {
                (candidates, r)
            }
        });
        let ogr = ogr.map(|v| Self::bin_to_dec(&v));
        let (_, co2sr) = (0..cs).fold((dr.clone(), None), |(candidates, r), i| {
            let lcb = Self::least_common(&candidates.iter().map(|r| r[i]).collect());
            if r == None {
                let n = candidates.into_iter().filter(|r| r[i] == lcb).collect::<Vec<_>>();
                (n.clone(), if n.len() == 1 { Some(n[0].clone()) } else { None })
            } else {
                (candidates, r)
            }
        });
        let co2sr = co2sr.map(|v| Self::bin_to_dec(&v));
        Ok(ogr.unwrap() * co2sr.unwrap())
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