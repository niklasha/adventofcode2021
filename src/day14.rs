use crate::day::*;
use crate::day::io::Read;
use regex::Regex;
use std::collections::HashMap;

pub struct Day14 {}

type Output = i64;

impl Day for Day14 {
    fn tag(&self) -> &str { "14" }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

impl Day14 {
    fn part1_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<Output> {
        self.process(input, 10)
    }

    fn part2_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<Output> {
        self.process(input, 40)
    }

    fn process(self: &Self, input: &mut dyn io::Read, steps: usize)
        -> BoxResult<Output> {
        let (init, rules) = Self::parse(input)?;
        let mut counts = HashMap::new();
        init.chars().for_each(|c| {
            counts.entry(c).and_modify(|count| *count += 1).or_insert(1);
        });
        let mut pair_counts = HashMap::new();
        init.chars().tuple_windows::<(_, _)>()
            .filter(|pair| rules.contains_key(pair)).for_each(|pair| {
            pair_counts.entry(pair).and_modify(|count| *count += 1)
                .or_insert(1);
        });
        (0..steps).fold(pair_counts, |pair_counts, _| {
            let mut new_pair_counts = HashMap::new();
            pair_counts.iter().for_each(|(pair, &pair_count)| {
                let new_pairs = rules.get(&pair).unwrap();
                let insert = new_pairs[0].1;
                new_pairs.iter().for_each(|&pair| {
                    new_pair_counts.entry(pair)
                        .and_modify(|count| *count += pair_count)
                        .or_insert(pair_count);
                });
                counts.entry(insert).and_modify(|count| *count += pair_count)
                    .or_insert(pair_count);
            });
            new_pair_counts
        });
        let counts = counts.into_iter().sorted_by_key(|&(_, count)| count)
            .collect::<Vec<_>>();
        Ok(counts[counts.len() - 1].1 - counts[0].1)
    }

    fn parse(input: &mut dyn Read)
        -> BoxResult<(String, HashMap<(char, char), [(char, char); 2]>)> {
        let mut lines = io::BufReader::new(input).lines();
        let init = lines.next().ok_or(AocError)??;
        lines.next().ok_or(AocError)??;
        let rules = lines.map(|line| line.map_err(|e| e.into())
            .and_then(Self::parse_insertion))
            .map(|r| r.unwrap()).map(|(k, v)| {
            let (left, right)
                = k.chars().tuples::<(_, _)>().next().unwrap();
            let insert = v.chars().next().unwrap();
            ((left, right), [(left, insert), (insert, right)])
        }).collect::<HashMap<_, _>>();
        Ok((init, rules))
    }

    fn parse_insertion(s: String) -> BoxResult<(String, String)> {
        lazy_static! {
            static ref RE: Regex = Regex::new("(\\w+) -> (\\w+)").unwrap();
        }
        let cap = RE.captures(&s).ok_or(AocError)?;
        Ok((cap[1].to_string(), cap[2].to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day14 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1("NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C", 1588);
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day14 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2("NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C", 2188189693529);
    }
}