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
    room: [[Option<char>; 2]; 4],
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
            Self::Room(no, _) => vec![Self::Room(*no, 0)],
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
    fn new(top: [char; 4], bottom: [char; 4]) -> Self {
        let mut burrow = Self { room: [[None; 2]; 4], hallway: [None; 11] };
        for i in 0..4 {
            burrow.room[i][0] = Some(top[i]);
            burrow.room[i][1] = Some(bottom[i]);
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
        &self, from: &Pos, exclude: Option<&Pos>, depth: usize, home_for: Option<char>)
        -> HashSet<(Pos, usize)> {
        let home_for
            = if depth == 0
            && match from { Pos::Hallway(_) => true, _ => false } {
            self.get(from)
        } else { home_for };
        let allowed_room = match from {
            Pos::Hallway(_) =>
                home_for.map_or(None, |amphipod| Some(Self::home(amphipod))),
            _ => None
        };
        if depth == 0 && self.is_immobile(from).unwrap() { HashSet::new() }
        else {
            let adjacent = from.adjacent();
            let adjacent = adjacent.iter()
                .filter(|to|
                    Some(*to) != exclude && self.get(to) == None
                    && allowed_room.map_or(
                        true,
                        |room| match *to {
                            Pos::Room(no, _) => *no == room,
                            _ => true
                        }))
                .collect::<HashSet<_>>();
            let moves = adjacent.iter()
                .flat_map(|to| self.moves(to, Some(from), depth + 1, home_for))
                .collect::<HashSet<_>>();
            if from.is_legal() && depth > 0 {
                moves.into_iter().chain(iter::once((from.clone(), depth))).collect()
            } else { moves }
                .iter().filter(
                    |(pos, _)|
                        home_for.map_or(true, |amphipod|
                            *pos == Pos::Room(Self::home(amphipod), 0)
                            || *pos == Pos::Room(Self::home(amphipod), 1)))
                .cloned().collect()
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
        match pos {
            Pos::Hallway(_) => {
                let amphipod = self.get(pos).ok_or(AocError)?;
                Ok(self.room[Self::home(amphipod)].iter()
                    .any(|space| *space != None && *space != Some(amphipod)))
            },
            _ => Ok(false)
        }
    }

    fn energy(&self, pos: &Pos) -> usize {
        match self.get(pos).unwrap() {
            'A' => 1, 'B' => 10, 'C' => 100, 'D' => 1000, _ => panic!("eek")
        }
    }

    fn are_all_home(&self) -> bool {
        self.room == [
            [Some('A'), Some('A')], [Some('B'), Some('B')],
            [Some('C'), Some('C')], [Some('D'), Some('D')]
        ]
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
        (0..4).map(|no| write!(f, "{}#", self.room[no][0].unwrap_or('.')))
            .collect::<fmt::Result>()?;
        write!(f, "##\n")?;
        write!(f, "  #")?;
        (0..4).map(|no| write!(f, "{}#", self.room[no][1].unwrap_or('.')))
            .collect::<fmt::Result>()?;
        write!(f, "\n")?;
        write!(f, "  #########\n")
    }
}

impl fmt::Debug for Burrow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Day23 {
   fn part1_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<Output> {
       let mut lines = io::BufReader::new(input).lines();
       lines.next();
       lines.next();
       let top = Self::parse(lines.next().ok_or(AocError)??)?;
       let bottom = Self::parse(lines.next().ok_or(AocError)??)?;
       let init = Burrow::new(top, bottom);
       let mut visited = iter::once((init.clone(), 0).clone())
           .collect::<HashMap<_, _>>();
       let mut heap = BinaryHeap::new();
       heap.push((usize::MAX, init));
       while let Some((energy, burrow)) = heap.pop() {
           let energy = usize::MAX - energy;
           println!("{} {} {}", heap.len(), energy, burrow);
           if burrow.are_all_home() { return Ok(energy) }
//           if energy == 440 { println!("moves {} {:?}", burrow, visited.get(&burrow)); }
           if visited.get(&burrow).map_or(false, |cost| energy > *cost) {
               continue;
           }
           let occupied = burrow.occupied();
           let moves = occupied.iter().flat_map(|pos|
               burrow.moves(&pos, None, 0, None).iter()
                   .map(|move_| (pos.clone(), move_.clone()))
                   .collect::<HashSet<_>>())
               .collect::<HashSet<_>>();
//           if energy == 440 { println!("moves {:?}", moves); }
           for (next, cost) in moves.iter()
               .map(|(pos, (to, distance))|
                   (burrow.move_(pos, to), burrow.energy(pos) * distance)) {
               if visited.get(&next).map_or(true, |dist| energy + cost < *dist) {
                   heap.push((usize::MAX - (energy + cost), next.clone()));
                   visited.insert(next.clone(), energy + cost);
               }
           }
       }
       println!("Failed");
       Err(Box::new(AocError))
    }

    fn part2_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<Output> {
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
        test2("", 0);
    }
}