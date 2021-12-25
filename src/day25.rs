use crate::day::*;
use closure::closure;

pub struct Day25 {}

type Output = i64;

impl Day for Day25 {
    fn tag(&self) -> &str { "25" }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

impl Day25 {
    fn part1_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let mut map = Utils::byte_matrix(input)?;
        let (h, w) = (map.len(), map[0].len());
        let n = (1..).map(|i| {
            let movable = map.iter().enumerate().flat_map(|(y, row)|
                row.iter().enumerate().filter(
                    closure!(move y, ref map, |&(x, cell)|
                        *cell == b'>' && map[y][(x + 1) % w] == b'.'))
                        .map(move |(x, _)| (x, y))).collect_vec();
            let no_move = movable.len() == 0;
            for (x, y) in movable {
                map[y][x] = b'.';
                map[y][(x + 1) % w] = b'>';
            }
            let movable = map.iter().enumerate().flat_map(|(y, row)|
                row.iter().enumerate().filter(
                    closure!(move y, ref map, |&(x, cell)|
                        *cell == b'v' && map[(y + 1) % h][x] == b'.'))
                        .map(move |(x, _)| (x, y))).collect_vec();
            let no_move = no_move && movable.len() == 0;
            for (x, y) in movable {
                map[y][x] = b'.';
                map[(y + 1) % h][x] = b'v';
            }
            if no_move { Some(i) } else { None }
        }).find(|n| n.is_some());
        Ok(n.ok_or(AocError)?.ok_or(AocError)?)
    }

    fn part2_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<Output> {
        Err(Box::new(AocError))
    }

    fn print(map: &Vec<Vec<u8>>) {
        for row in map {
            println!("{}",
                     row.iter().map(|cell| *cell as char).collect::<String>());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day25 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1("v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>", 58);
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day25 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2("", 0);
    }
}