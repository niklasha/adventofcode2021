use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use crate::day::*;

pub struct Day08 {}

type Output = usize;

impl Day for Day08 {
    fn tag(&self) -> &str { "08" }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

impl Day08 {
    fn part1_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let lines = io::BufReader::new(input).lines()
            .map(|l| Self::parse(&l.unwrap()).unwrap())
            .collect::<Vec<_>>();
        Ok(lines.iter()
            .map(|(_, output)|
                output.iter().filter(|digit| [2usize, 3, 4, 7].contains(&digit.len()))
                    .count())
            .sum())
    }

    fn part2_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let lines = io::BufReader::new(input).lines()
            .map(|l| Self::parse(&l.unwrap()).unwrap())
            .collect::<Vec<_>>();
        let segments = vec![
            ("abcefg", 0), ("cf", 1), ("acdeg", 2), ("acdfg", 3), ("bcdf", 4),
            ("abdfg", 5), ("abdefg", 6), ("acf", 7), ("abcdefg", 8),
            ("abcdfg", 9)
        ].into_iter().collect::<HashMap<_, _>>();
        let translations
            = "abcdefg".chars().permutations(7).collect::<Vec<_>>();
        Ok(lines.iter()
            .map(|(input, output)| {
                let translation
                    = translations.iter().find(|candidate| input.iter()
                    .all(|digit|
                        segments.contains_key(
                            Self::translate(&digit, candidate).as_str())))
                    .unwrap();
                output.iter()
                    .map(|digit|
                        segments.get(
                            Self::translate(&digit, &translation).as_str())
                            .unwrap())
                    .fold(0, |s, n| s * 10 + n)
            })
            .sum())
    }

    fn parse(s: &str) -> BoxResult<(Vec<String>, Vec<String>)> {
        lazy_static! {
            static ref RE: Regex
                = Regex::new(
                    "(\\w+) (\\w+) (\\w+) (\\w+) (\\w+) (\\w+) (\\w+) (\\w+) (\\w+) (\\w+) \\| (\\w+) (\\w+) (\\w+) (\\w+)")
                .unwrap();
        }
        let cap = RE.captures(s).ok_or(AocError)?;
        Ok(
            (vec![cap[1].parse()?, cap[2].parse()?,
                cap[3].parse()?, cap[4].parse()?,
                cap[5].parse()?, cap[6].parse()?,
                cap[7].parse()?, cap[8].parse()?,
                cap[9].parse()?, cap[10].parse()?],
            vec![cap[11].parse()?, cap[12].parse()?,
                cap[13].parse()?, cap[14].parse()?]))
    }

    fn translate(s: &str, key: &Vec<char>) -> String {
        s.chars().map(|c| key[c as usize - 'a' as usize]).sorted().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day08 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1(
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
            26);
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day08 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2(
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
            61229);
    }
}