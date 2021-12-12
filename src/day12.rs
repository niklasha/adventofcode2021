use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::iter;
use std::str::FromStr;

use itertools::FoldWhile::{Continue, Done};
use regex::Regex;

use crate::day::*;
use crate::day::io::Read;

pub struct Day12 {}

type Output = usize;

impl Day for Day12 {
    fn tag(&self) -> &str { "12" }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
enum Vertex {
    Start,
    End,
    Small(String),
    Big(String)
}

impl FromStr for Vertex {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "start" => Self::Start,
            "end" => Self::End,
            _ if s.chars().next().ok_or(AocError)?.is_lowercase() =>
                Self::Small(s.to_string()),
            _ => Self::Big(s.to_string())
        })
    }
}

type Path = Vec<Vertex>;
type Graph = HashMap<Vertex, HashSet<Vertex>>;

trait State {
    fn path_mut(&mut self) -> &mut Path;
    fn path(&self) -> &Path;
    fn next_state(&self, candidate: &Vertex) -> Option<Box<Self>>;
}

impl State for Path {
    fn path_mut(&mut self) -> &mut Path { self }

    fn path(&self) -> &Path { self }

    fn next_state(&self, candidate: &Vertex) -> Option<Box<Self>> {
        if match candidate {
            Vertex::Small(_) =>
                self.path().contains(&candidate),
            _ => false
        } { None } else {
            let mut new_state = self.clone();
            new_state.path_mut().push(candidate.clone());
            Some(Box::new(new_state))
        }
    }
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct State2(Path, Option<Vertex>);
impl State for State2 {
    fn path_mut(&mut self) -> &mut Path { &mut self.0 }

    fn path(&self) -> &Path { &self.0 }

    fn next_state(&self, candidate: &Vertex) -> Option<Box<Self>> {
        let chosen_vertex = &self.1;
        let (block, chosen_vertex) = match candidate {
            Vertex::Small(_) => {
                let visits = self.path().iter()
                    .filter(|&vertex| vertex == candidate).count();
                let is_second_ok
                    = *chosen_vertex == None
                    || chosen_vertex.as_ref() == Some(candidate);
                (visits > 1 || visits == 1 && !is_second_ok.clone(),
                 if visits == 1 && is_second_ok { Some(candidate.clone()) }
                 else { chosen_vertex.clone() })
            },
            _ => (false, chosen_vertex.clone())
        };
        if block { None } else {
            let mut new_path = self.path().clone();
            new_path.push(candidate.clone());
            Some(Box::new(State2(new_path, chosen_vertex.clone())))
        }
    }
}

impl Day12 {
    fn part1_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<Output> {
        Ok(Self::traverse(&Self::get_graph(input), vec![Vertex::Start]).len())
    }

    fn part2_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<Output> {
        Ok(Self::traverse(
            &Self::get_graph(input), State2(vec![Vertex::Start], None))
            .len())
    }

    fn get_graph(input: &mut dyn Read) -> Graph {
        io::BufReader::new(input).lines()
            .map(|l| Self::parse(&l.unwrap()).unwrap())
            .fold(HashMap::new(), |mut graph, (a, b)| {
                let mut add = |a: &Vertex, b: &Vertex| {
                    graph.entry(a.clone())
                        .and_modify(
                            |neighbours| { (*neighbours).insert(b.clone()); })
                        .or_insert(iter::once(b.clone()).collect::<HashSet<_>>());
                };
                if a != Vertex::End && b != Vertex::Start { add(&a, &b); }
                if b != Vertex::End && a != Vertex::Start { add(&b, &a); }
                graph
            })
    }

    // init example:
    fn traverse<S>(graph: &Graph, init: S) -> HashSet<S>
        where S: Hash + Eq + State + Clone {
        (0..).fold_while(
            iter::once(init).collect::<HashSet<_>>(),
            |states, _| {
                let new_states = states.iter().flat_map(|state| {
                    let last = state.path().last().unwrap();
                    if *last == Vertex::End { vec![state.clone()] }
                    else {
                        let candidates = graph.get(&last);
                        if candidates.is_none() { vec![] } else {
                            candidates.unwrap().iter().flat_map(|candidate|
                                state.next_state(candidate).map(|s| *s))
                                .collect()
                        }
                    }
                }).collect::<HashSet<_>>();
                if new_states.iter().all(|state| state.path().last() == Some(&Vertex::End))
                    { Done(new_states) }
                else { Continue(new_states) }
            })
            .into_inner()
    }

    fn parse(s: &str) -> BoxResult<(Vertex, Vertex)> {
        lazy_static! {
            static ref RE: Regex= Regex::new("(\\w+)-(\\w+)").unwrap();
        }
        let cap = RE.captures(s).ok_or(AocError)?;
        Ok((Vertex::from_str(&cap[1])?, Vertex::from_str(&cap[2])?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day12 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1("start-A
start-b
A-c
A-b
b-d
A-end
b-end", 10);
        test1("dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc", 19);
        test1("fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW", 226);
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day12 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2("start-A
start-b
A-c
A-b
b-d
A-end
b-end", 36);
        test2("dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc", 103);
        test2("fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW", 3509);
    }
}