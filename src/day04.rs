use itertools::Itertools;
use itertools::FoldWhile::*;
use crate::day::*;
//use crate::day::io::Read;

pub struct Day04 {}

type Output = i64;

impl Day for Day04 {
    fn tag(&self) -> &str { "04" }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

type Board = Vec<Vec<(Output, bool)>>;

impl Day04 {
    fn part1_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let mut lines = io::BufReader::new(input).lines();
        let numbers = lines.next().ok_or(Box::new(AocError))??;
        let mut numbers = numbers.split(",")
            .map(|s| s.parse::<Output>().unwrap());
        lines.next();
        let boards = lines.map(|s| s.unwrap().split_whitespace()
            .map(|s| (s.parse::<Output>().unwrap(), false))
            .collect::<Vec<_>>())
            .map(|v| vec![v])
            .coalesce(|x, y|
                if y[0].is_empty() { Err((x, y)) } else if x[0].is_empty() { Ok(y) } else { Ok(x.into_iter().chain(y).collect::<Vec<_>>()) })
            .collect::<Vec<_>>();
//        let (numbers, boards) = Self::setup(input)?;
        let (win, last) = numbers.fold_while(
            (boards, None), |(boards, _), n| {
                let boards = boards.into_iter().map(move |b| Self::mark(b, n))
                    .collect::<Vec<_>>();
                let win = boards.iter().find(|&b| Self::bingo(b));
                match win {
                    Some(b) => Done((vec![b.clone()], Some(n))),
                    None => Continue((boards, Some(n)))
                }
        }).into_inner();
        let sum: Output = win[0].iter()
            .map(|r|
                r.iter().map(|&(n, m)| if m { 0 } else { n }).sum::<Output>())
            .sum();
        Ok(sum * last.ok_or(Box::new(AocError))?)
    }

    fn part2_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let mut lines = io::BufReader::new(input).lines();
        let numbers = lines.next().ok_or(Box::new(AocError))??;
        let mut numbers = numbers.split(",")
            .map(|s| s.parse::<Output>().unwrap());
        lines.next();
        let boards = lines.map(|s| s.unwrap().split_whitespace()
            .map(|s| (s.parse::<Output>().unwrap(), false))
            .collect::<Vec<_>>())
            .map(|v| vec![v])
            .coalesce(|x, y|
                if y[0].is_empty() { Err((x, y)) } else if x[0].is_empty() { Ok(y) } else { Ok(x.into_iter().chain(y).collect::<Vec<_>>()) })
            .collect::<Vec<_>>();
//        let (numbers, boards) = Self::setup(input)?;
        let (win, last) = numbers.fold_while(
            (boards, None), |(boards, _), n| {
                let boards = boards.into_iter().map(move |b| Self::mark(b, n))
                    .collect::<Vec<_>>();
                let foo = boards.iter()
                    .map(|b|
                         if Self::bingo(&b) { (Some(b), None) }
                         else { (None, Some(b)) })
                    .collect::<Vec<_>>();
                let win = foo.iter().flat_map(|t| t.0).collect::<Vec<_>>();
                let no_win = foo.iter().flat_map(|t| t.1).collect::<Vec<_>>();
                if !win.is_empty() && no_win.is_empty() {
                    Done((vec![win[0].clone()], Some(n)))
                } else {
                    Continue((no_win.into_iter().map(|b| b.clone()).collect(), Some(n)))
                }
            }).into_inner();
        let sum: Output = win[0].iter()
            .map(|r|
                r.iter().map(|&(n, m)| if m { 0 } else { n }).sum::<Output>())
            .sum();
        Ok(sum * last.ok_or(Box::new(AocError))?)
    }

    // fn setup(input: &mut dyn Read) -> BoxResult<(Iterator<Item=Output>, Vec<Board>)> {
    //     let mut lines = io::BufReader::new(input).lines();
    //     let numbers = lines.next().ok_or(Box::new(AocError))??;
    //     let mut numbers = numbers.split(",")
    //         .map(|s| s.parse::<Output>().unwrap());
    //     lines.next();
    //     let boards = lines.map(|s| s.unwrap().split_whitespace()
    //         .map(|s| (s.parse::<Output>().unwrap(), false))
    //         .collect::<Vec<_>>())
    //         .map(|v| vec![v])
    //         .coalesce(|x, y|
    //             if y[0].is_empty() { Err((x, y)) } else if x[0].is_empty() { Ok(y) } else { Ok(x.into_iter().chain(y).collect::<Vec<_>>()) })
    //         .collect::<Vec<_>>();
    //     Ok((numbers, boards))
    // }

    fn mark(b: Board, n: Output) -> Board {
        b.into_iter().map(move |r| r.into_iter()
            .map(move |(x, m)| (x, m || x == n)).collect()).collect()
    }

    fn mirror(b: &Board) -> Board {
        let sz = b.len();
        let mut m = vec![vec![(-1, false); sz]; sz];
        (0..sz).for_each(|x| (0..sz).for_each(|y| m[x][y] = b[y][x]));
        m
    }

    fn bingo(b: &Board) -> bool {
        b.iter().any(|r| r.iter().all(|&(_, m)| m))
            || Self::mirror(b).iter().any(|r| r.iter().all(|&(_, m)| m))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day04 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1("7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
",
              4512);
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day04 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2("7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
",
              1924);
    }
}