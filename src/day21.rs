use crate::day::*;
use std::cmp;
use std::collections::HashMap;

pub struct Day21 {}

type Output = i64;

impl Day for Day21 {
    fn tag(&self) -> &str { "21" }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

impl Day21 {
    fn part1_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let mut lines = io::BufReader::new(input).lines();
        let p1: Output = lines.next().ok_or(AocError)??.split(" ").nth(4).ok_or(AocError)?.parse()?;
        let p2: Output = lines.next().ok_or(AocError)??.split(" ").nth(4).ok_or(AocError)?.parse()?;
        let mut die = 1;
        let mut i = 0;
        let mut score = [0, 0];
        let mut pos = [p1, p2];
        loop {
            if score[0] >= 1000 || score[1] >= 1000 { break; }
            let sum = die + (die % 100 + 1) + ((die + 1) % 100 + 1);
            pos[i % 2] = (pos[i % 2] - 1 + sum) % 10 + 1;
            score[i % 2] += pos[i % 2];
            die = (die + 2) % 100 + 1;
            i += 1
        }
        Ok((i * 3) as Output * cmp::min(score[0], score[1]))
    }

    fn part2_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let mut lines = io::BufReader::new(input).lines();
        let p1: usize = lines.next().ok_or(AocError)??.split(" ").nth(4).ok_or(AocError)?.parse()?;
        let p2: usize = lines.next().ok_or(AocError)??.split(" ").nth(4).ok_or(AocError)?.parse()?;
        let mut i = 0;
        let mut worlds = HashMap::new();
        worlds.insert(([0 as usize, 0 as usize], [p1 - 1, p2 - 1]), 1 as Output);
        let rolls = Self::rolls();
        while worlds.iter()
            .any(|((score, _), _)| score[0] < 21 && score[1] < 21) {
            let mut insert_queue = vec![];
            let mut remove_queue = vec![];
            for ((score, pos), cnt) in &worlds {
                if score[0] < 21 && score[1] < 21 {
                    remove_queue.push((*score, *pos));
                    for roll in &rolls {
                        let mut pos = pos.clone();
                        pos[i % 2] = (pos[i % 2] + roll) % 10;
                        let mut score = score.clone();
                        score[i % 2] += pos[i % 2] + 1;
                        insert_queue.push(((score, pos), cnt.clone()));
                    }
                }
            }
            worlds.retain(|k, _| !remove_queue.contains(k));
            for (k, cnt) in insert_queue {
                worlds.entry(k).and_modify(|e| *e += cnt).or_insert(cnt);
            }
            i += 1;
        }
        Ok(*worlds.into_iter()
            .fold([0, 0], |sum, ((score, _), cnt)|
                if score[0] > score[1] { [sum[0] + cnt, sum[1]] }
                else { [sum[0], sum[1] + cnt] })
            .iter().max().ok_or(AocError)?)
    }

    fn rolls() -> Vec<usize> {
        (1..=3).flat_map(|i|
            (1..=3).flat_map(move |j|
                (1..=3).map(move |k| i + j + k))).collect_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day21 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1("Player 1 starting position: 4
Player 2 starting position: 8",
              739785);
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day21 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2("Player 1 starting position: 4
Player 2 starting position: 8",
              444356092776315);
    }
}