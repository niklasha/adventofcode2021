use crate::day::*;
use itertools::FoldWhile::{Done, Continue};
use regex::Regex;
use std::collections::{HashMap, HashSet};

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

impl Vertex {
    fn from_str(s: String) -> BoxResult<Vertex> {
        Ok(match s.as_str() {
            "start" => Self::Start,
            "end" => Self::End,
            _ if s.chars().next().ok_or(AocError)?.is_lowercase() =>
                Self::Small(s),
            _ => Self::Big(s)
        })
    }

    fn to_str(&self) -> &str {
        match self {
            Self::Start => "start",
            Self::End => "end",
            Self::Small(s) => &s,
            Self::Big(s) => &s,
        }
    }
}

type Path = Vec<Vertex>;
type Graph = HashMap<Vertex, HashSet<Vertex>>;

impl Day12 {
    fn part1_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let edges = io::BufReader::new(input).lines()
            .map(|l| Self::parse(&l.unwrap()).unwrap())
            .collect::<Vec<_>>();
        let mut vertices: HashSet<Vertex> = HashSet::new();
        let mut graph: Graph = HashMap::new();
        for (a, b) in edges {
            if !vertices.contains(&a) { vertices.insert(a.clone()); }
            if !vertices.contains(&b) { vertices.insert(b.clone()); }
            Self::connect(&mut graph, a, b);
        }
        let paths = (0..).fold_while(
            vec![vec![Vertex::Start]].into_iter().collect::<HashSet<Path>>(),
            |paths, _| {
                let new_paths = paths.iter().flat_map(|path| {
                    let last = path.last().unwrap();
                    if *last == Vertex::End { vec![path.clone()] }
                    else {
                        let candidates = graph.get(&last);
                        if candidates.is_none() { vec![] } else {
                            candidates.unwrap().iter().flat_map(|candidate|
                                if match candidate {
                                    Vertex::Small(_) =>
                                        path.contains(&candidate),
                                    _ => false
                                } { None } else {
                                    let mut new_path = path.clone();
                                    new_path.push(candidate.clone());
                                    Some(new_path)
                                })
                                .collect()
                        }
                    }
                }).collect::<HashSet<_>>();
                if new_paths.iter().all(|path| path.last() == Some(&Vertex::End))
                { Done(new_paths) }
                else { Continue(new_paths) }
            })
            .into_inner();
        Ok(paths.len())
    }

    fn part2_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let edges = io::BufReader::new(input).lines()
            .map(|l| Self::parse(&l.unwrap()).unwrap())
            .collect::<Vec<_>>();
        let mut vertices: HashSet<Vertex> = HashSet::new();
        let mut graph: Graph = HashMap::new();
        for (a, b) in edges {
            if !vertices.contains(&a) { vertices.insert(a.clone()); }
            if !vertices.contains(&b) { vertices.insert(b.clone()); }
            Self::connect(&mut graph, a, b);
        }
        let paths = (0..).fold_while(
            vec![(vec![Vertex::Start], None)].into_iter()
                .collect::<HashSet<(Path, Option<Vertex>)>>(),
            |states, i| {
                let new_states = states.iter().flat_map(|(path, chosen_vertex)| {
                    let last = path.last().unwrap();
                    if *last == Vertex::End { vec![(path.clone(), chosen_vertex.clone())] }
                    else {
                        let candidates = graph.get(&last);
                        if candidates.is_none() { vec![] } else {
                            candidates.unwrap().iter().flat_map(|candidate| {
                                let (block, chosen_vertex) = match candidate {
                                    Vertex::Small(_) => {
                                        let visits = path.iter()
                                            .filter(|&vertex| vertex == candidate).count();
                                        let is_second_ok
                                            = *chosen_vertex == None
                                            || chosen_vertex.as_ref() == Some(candidate);
                                        (visits > 1 || visits == 1 && !is_second_ok,
                                         if visits == 1 && is_second_ok { Some(candidate.clone()) } else { chosen_vertex.clone() })
                                    },
                                    _ => (false, chosen_vertex.clone())
                                };
                                if block { None } else {
                                    let mut new_path = path.clone();
                                    new_path.push(candidate.clone());
                                    Some((new_path, chosen_vertex.clone()))
                                }
                            }).collect()
                        }
                    }
                }).collect::<HashSet<(_, _)>>();
                if new_states.iter().all(|(path, _)| path.last() == Some(&Vertex::End))
                    { Done(new_states) }
                else { Continue(new_states) }
            })
            .into_inner();
        Ok(paths.len())
    }

    fn connect(graph: &mut Graph, a: Vertex, b: Vertex) {
        if a != Vertex::End && b != Vertex::Start {
            if !graph.contains_key(&a) {
                let connections = HashSet::new();
                graph.insert(a.clone(), connections);
            }
            // XXX Is there a way to not clone here?
            // XXX Is it not generating overhead?
            graph.get_mut(&a).unwrap().insert(b.clone());
        }
        if b != Vertex::End && a != Vertex::Start {
            if !graph.contains_key(&b) {
                let connections = HashSet::new();
                graph.insert(b.clone(), connections);
            }
            graph.get_mut(&b).unwrap().insert(a);
        }
    }

    fn parse(s: &str) -> BoxResult<(Vertex, Vertex)> {
        lazy_static! {
            static ref RE: Regex= Regex::new("(\\w+)-(\\w+)").unwrap();
        }
        let cap = RE.captures(s).ok_or(AocError)?;
        Ok((Vertex::from_str(cap[1].to_string())?,
            Vertex::from_str(cap[2].to_string())?))
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