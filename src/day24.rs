use crate::day::*;
use crate::cpu::Cpu;
use evmap::{self, ReadHandle, WriteHandle};
use std::cmp;
use std::iter;

pub struct Day24 {}

type Output = i64;

impl Day for Day24 {
    fn tag(&self) -> &str { "24" }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

impl Day24 {
    fn part1_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<Output> {
        Self::process(input, true)
    }

    fn part2_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<Output> {
        Self::process(input, false)
    }

    fn process(input: &mut dyn io::Read, desc: bool) -> BoxResult<Output> {
        let (start, stop, dir) = if desc { (9, 1, -1) } else { (1, 9, 1) };
//        let mut cpu = Cpu::from(input)?;
        let mut model = vec![start; 14];
        let limit = model.len();
        let mut i = limit - 1;
        let (cache_r, mut cache_w) = evmap::new();
        while i >= 0 {
//            let (_, reg) = cpu.run([0; 4], &model)?;
            if let Some(j) = Self::check(&cache_r, &mut cache_w, &model, 0) {
                for k in (j + 1)..limit { model[k] = start }
                for j in (i..=j).rev() {
                    if (model[j] - stop).signum() != dir { model[j] += dir; break; }
                    model[j] = start;
                }
                if (i..limit).all(|j| model[j] == start) {
                    i -= 1;
                    model[i] += dir
                }
            } else {
                return Ok(model.iter().fold(0, |sum, d| sum * 10 + d));
            }
        }
        Err(Box::new(AocError))
    }

    // XXX Manually transcribed logic from my input
    fn check(cache_r: &ReadHandle<(usize, i64), i64>,
             cache_w: &mut WriteHandle<(usize, i64), i64>,
             model: &Vec<i64>, z: i64)
             -> Option<usize> {
        let mut z = z;
        let mut zv = vec![];
        for i in 0..14 {
            let p = [
                (1, 15, 15),
                (1, 12, 5),
                (1, 13, 6),
                (26, -14, 7),
                (1, 15, 9),
                (26, -7, 6),
                (1, 14, 14),
                (1, 15, 3),
                (1, 15, 1),
                (26, -7, 3),
                (26, -8, 4),
                (26, -7, 6),
                (26, -5, 7),
                (26, -10, 1)
            ];
            z = Self::alu_part(model[i], z, p[i].0, p[i].1, p[i].2);
            if i > 8 && z > 26i64.pow((13 - i) as u32)
                || i > 5 && z > 26i64.pow(5) {
//                println!("pruning {:?} {} {}", model, i, z);
                zv.push(z);
                let i = Self::mark(cache_r, cache_w, i, &mut zv, model);
                cache_w.refresh();
                return Some(i);
            }
            if let Some(visited) = cache_r.get(&(i, z)) {
                if visited.contains(&model[i]) {
                    return Some(i)
                }
            }
            zv.push(z);
        }
        if z == 0 { None } else {
            let i = Self::mark(cache_r, cache_w, 13, &mut zv, model);
            cache_w.refresh();
            Some(i)
        }
    }

    fn mark(cache_r: &evmap::ReadHandle<(usize, i64), i64>,
            cache_w: &mut evmap::WriteHandle<(usize, i64), i64>,
            i: usize, zv: &Vec<i64>, model: &Vec<i64>)
            -> usize {
        if let Some(visited) = cache_r.get(&(i, zv[i])) {
            let visited = visited.iter().collect_vec();
            if !visited.contains(&&model[i]) {
                if visited.len() < 8 {
                    cache_w.insert((i, zv[i]), model[i]);
                } else {
//                    println!("mark_2 recursion at {:?} {}", model, i);
                    visited.iter()
                        .for_each(|d| { cache_w.remove((i, zv[i]), **d); });
                    return Self::mark(cache_r, cache_w, i - 1, zv, model);
                }
            }
        } else { cache_w.insert((i, zv[i]), model[i]); }
        i
    }

    // XXX Manually transcribed logic from my input
    fn alu_part(w: i64, z: i64, a:i64, b: i64, c: i64) -> i64 {
        if z % 26 + b == w { z / a } else { z / a * 26 + c + w }
    }

}
