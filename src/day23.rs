use crate::day::*;
use regex::Regex;
use std::collections::{HashSet, HashMap, BinaryHeap};
use std::{fmt, iter};

pub struct Day23 {}

type Output = usize;

impl Day for Day23 {
    fn tag(&self) -> &str { "23" }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Burrow {
    room: [Vec<Option<char>>; 4],
    hallway: [Option<char>; 11]
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Pos {
    Room(usize, usize),
    Hallway(usize)
}

impl Pos {
    fn adjacent(&self) -> HashSet<Self> {
        match self {
            Self::Room(no, 0) =>
                vec![Self::Room(*no, 1), Self::Hallway(2 + *no * 2)],
            Self::Room(no, level) =>
                vec![Self::Room(*no, *level - 1), Self::Room(*no, *level + 1)],
            Self::Hallway(0) => vec![Self::Hallway(1)],
            Self::Hallway(10) => vec![Self::Hallway(9)],
            Self::Hallway(i) =>
                if *i % 2 == 0 {
                    vec![
                        Self::Hallway(*i - 1), Self::Hallway(*i + 1),
                        Self::Room(*i / 2 - 1, 0),
                    ]
                } else {
                    vec![Self::Hallway(*i - 1), Self::Hallway(*i + 1)]
                }
        }.iter().cloned().collect()
    }

    fn is_legal(&self) -> bool {
        *self != Self::Hallway(2) && *self != Self::Hallway(4)
            && *self != Self::Hallway(6) && *self != Self::Hallway(8)
    }
}

impl Burrow {
    fn new(level: Vec<[char; 4]>) -> Self {
        let mut burrow = Self { room: Default::default(), hallway: [None; 11] };
        for level in level.iter() {
            for i in 0..4 {
                burrow.room[i].push(Some(level[i]));
            }
        }
        burrow
    }

    fn get(&self, pos: &Pos) -> Option<char> {
        match pos {
            Pos::Room(no, level) => self.room[*no][*level],
            Pos::Hallway(i) => self.hallway[*i]
        }
    }

    fn get_mut(&mut self, pos: &Pos) -> &mut Option<char> {
        match pos {
            Pos::Room(no, level) => &mut self.room[*no][*level],
            Pos::Hallway(i) => &mut self.hallway[*i]
        }
    }

    fn moves(
        &self, from: &Pos, exclude: Option<&Pos>, depth: usize, origin: &Pos)
        -> HashSet<(Pos, usize)> {
        let amphipod = self.get(origin).unwrap();
        if depth == 0 && self.is_immobile(origin).unwrap() { HashSet::new() }
        else {
            let adjacent = from.adjacent();
            let adjacent = adjacent.iter().filter(
                |space| match space {
                    Pos::Room(_, level) => *level < self.room[0].len(),
                    _ => true
                }).collect::<HashSet<_>>();
            let adjacent = adjacent.into_iter()
                .filter(|&to|
                    Some(to) != exclude && self.get(to) == None
                        && Self::acceptable(from, to, amphipod))
                .collect::<HashSet<_>>();
            let moves = adjacent.iter()
                .flat_map(|to| self.moves(to, Some(from), depth + 1, origin))
                .collect::<HashSet<_>>();
            if depth > 0 && from.is_legal() && !self.no_stop(origin, from) {
                moves.into_iter().chain(iter::once((from.clone(), depth))).collect()
            } else { moves }
                .iter().cloned().collect()
        }
    }

    fn acceptable(from: &Pos, to: &Pos, amphipod: char) -> bool {
        match (from, to) {
            (Pos::Hallway(_), Pos::Room(no, _)) => *no == Self::home(amphipod),
            _ => true,
        }
    }

    fn no_stop(&self, origin: &Pos, from: &Pos) -> bool {
        match (origin, from) {
            (Pos::Hallway(_), Pos::Hallway(_)) => true,
            (_, Pos::Room(no, level)) =>
                *level < self.room[0].len() - 1
                    && !((*level + 1)..self.room[0].len()).all(|level|
                    self.room[*no][level] == self.get(origin)),
            _ => false
        }
    }

    fn occupied(&self) -> HashSet<Pos> {
        self.hallway.iter().enumerate()
            .flat_map(|(i, space)| space.map(|_| Pos::Hallway(i)))
            .chain(self.room.iter().enumerate()
                .flat_map(|(no, room)|
                    room.iter().enumerate().flat_map(
                        move |(i, space)| space.map(|_| Pos::Room(no, i)))))
        .collect()
    }

    fn move_(&self, from: &Pos, to: &Pos) -> Self {
        let mut burrow = self.clone();
        *burrow.get_mut(to) = burrow.get(from);
        *burrow.get_mut(from) = None;
        burrow
    }

    fn home(amphipod: char) -> usize { amphipod as usize - 'A' as usize }

    fn is_immobile(&self, pos: &Pos) -> BoxResult<bool> {
        let amphipod = self.get(pos).ok_or(AocError)?;
        Ok(match pos {
            Pos::Hallway(_) => {
                self.room[Self::home(amphipod)].iter()
                    .any(|space| *space != None && *space != Some(amphipod))
            },
            Pos::Room(no, level) =>
                *no == Self::home(amphipod)
                    && (*level == self.room[0].len() - 1
                        || ((*level + 1)..self.room[0].len()).all(|level|
                            self.room[*no][level] == Some(amphipod)))
                || (self.hallway[*no * 2 + 1] != None
                    && self.hallway[*no * 2 + 3] != None)
        })
    }

    fn energy(&self, pos: &Pos) -> usize {
        match self.get(pos).unwrap() {
            'A' => 1, 'B' => 10, 'C' => 100, 'D' => 1000, _ => panic!("eek")
        }
    }

    fn are_all_home(&self) -> bool {
        (0..4).all(|no|
            self.room[no].iter().all(|space|
                space.map_or(false, |amphipod| Self::home(amphipod) == no)))
    }
}

impl fmt::Display for Burrow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n#############\n")?;
        write!(f, "#")?;
        self.hallway.iter().map(|space| write!(f, "{}", space.unwrap_or('.')))
            .collect::<fmt::Result>()?;
        write!(f, "#\n")?;
        write!(f, "###")?;
        for level in 0..self.room[0].len() {
            (0..4).map(|no| write!(f, "{}#", self.room[no][level].unwrap_or('.')))
                .collect::<fmt::Result>()?;
            write!(f, "{}\n", if level == 0 { "##" } else { "" })?;
            write!(f, "  #")?;
        }
        write!(f, "########\n")
    }
}

impl fmt::Debug for Burrow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Day23 {
   fn part1_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<Output> {
       Self::process(input, false)
    }

    fn part2_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<Output> {
        Self::process(input, true)
    }

    fn process(input: &mut dyn io::Read, fold: bool) -> BoxResult<Output> {
        let mut lines = io::BufReader::new(input).lines();
        lines.next();
        lines.next();
        let top = Self::parse(lines.next().ok_or(AocError)??)?;
        let bottom = Self::parse(lines.next().ok_or(AocError)??)?;
        let init = Burrow::new(
            if fold {
                vec![top, ['D', 'C', 'B', 'A'], ['D', 'B', 'A', 'C'], bottom]
            } else { vec![top, bottom] });
        let mut visited = iter::once((init.clone(), 0).clone())
            .collect::<HashMap<_, _>>();
        let mut heap = BinaryHeap::new();
        heap.push((usize::MAX, init));
        while let Some((energy, burrow)) = heap.pop() {
            let energy = usize::MAX - energy;
            if burrow.are_all_home() { return Ok(energy) }
            if visited.get(&burrow).map_or(false, |cost| energy > *cost) {
                continue;
            }
            let occupied = burrow.occupied();
            let moves = occupied.iter().flat_map(|pos|
                burrow.moves(&pos, None, 0, &pos).iter()
                    .map(|move_| (pos.clone(), move_.clone()))
                    .collect::<HashSet<_>>())
                .collect::<HashSet<_>>();
            let moves = moves.iter().filter(|(from, (to, _))| to.is_legal() && !burrow.no_stop(from, to)).collect::<HashSet<_>>();
            for (next, cost) in moves.iter()
                .map(|(pos, (to, distance))|
                    (burrow.move_(pos, to), burrow.energy(pos) * distance)) {
                if visited.get(&next).map_or(true, |dist| energy + cost < *dist) {
                    heap.push((usize::MAX - (energy + cost), next.clone()));
                    visited.insert(next.clone(), energy + cost);
                }
            }
        }
        Err(Box::new(AocError))
    }

    fn parse(s: String) -> BoxResult<[char; 4]> {
        lazy_static! {
            static ref RE: Regex
                = Regex::new("...([ABCD]).([ABCD]).([ABCD]).([ABCD]).*")
                .unwrap();
        }
        let cap = RE.captures(&s).ok_or(AocError)?;
        Ok([
            cap[1].chars().next().ok_or(AocError)?,
            cap[2].chars().next().ok_or(AocError)?,
            cap[3].chars().next().ok_or(AocError)?,
            cap[4].chars().next().ok_or(AocError)?
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day23 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1("#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########",
              12521);
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day23 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2("#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########",
              44169);
    }
}