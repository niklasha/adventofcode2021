use crate::day::*;
use itertools::Itertools;
use na::{Rotation, Vector3};
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fmt;
use crate::day::io::Read;

pub struct Day19 {}

type Output = usize;

impl Day for Day19 {
    fn tag(&self) -> &str { "19" }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Pos([i64; 3]);

impl Pos {
    fn parse(s: &str) -> BoxResult<Self> {
        let mut split = s.split(",");
        Ok(Pos([
            split.next().ok_or(AocError)?.parse()?,
            split.next().ok_or(AocError)?.parse()?,
            split.next().ok_or(AocError)?.parse()?]))
    }
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{},{}", self.0[0], self.0[1], self.0[2])
    }
}

impl Day19 {
    fn part1_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let beacons = Day19::parse(input)?;
        let rotations = &Self::rotations();
        let scanner_pairs = beacons.keys().combinations(2)
            .map(|v| v.into_iter().cloned().sorted().collect_vec())
            .sorted().collect_vec();
        let mut scanner_pos = HashMap::new();
        let mut scanner_rotation = HashMap::new();
        for pair in scanner_pairs {
            let s0_beacons = beacons.get(&pair[0]).unwrap();
            let s1_beacons = beacons.get(&pair[1]).unwrap();
            let mut s0_s1_candidates: HashMap<(usize, Pos), usize>
                = HashMap::new();
            for s0_b in s0_beacons {
                let s0_b = &Vector3::from_iterator(
                    s0_b.0.iter().map(|i| *i as f32));
                for s1_b in s1_beacons {
                    let s1_b = &Vector3::from_iterator(
                        s1_b.0.iter().map(|i| *i as f32));
                    rotations.iter().enumerate().for_each(|(i, r)| {
                        let s0_s1 = s0_b - r.transform_vector(s1_b);
                        let key
                            = (i,
                               Pos([
                                   s0_s1.x as i64,
                                   s0_s1.y as i64,
                                   s0_s1.z as i64
                               ]));
                        s0_s1_candidates.entry(key).and_modify(|e| *e += 1)
                            .or_insert(1);
                    });
                }
            }
            let found = &s0_s1_candidates.iter().filter(|&(_, n)| *n >= 12)
                .collect_vec();
            if !found.is_empty() {
                scanner_rotation.insert(pair.clone(), found[0].0.0.clone());
                scanner_pos.insert(pair.clone(), found[0].0.1.clone());
            }
        }
        let mut scanner_parent = HashMap::new();
        scanner_parent.insert("--- scanner 0 ---", None);
        while beacons.keys().any(|scanner| !scanner_parent.contains_key(scanner.as_str())) {
            scanner_rotation.keys().for_each(|v| {
                if scanner_parent.contains_key(v[0].as_str())
                    && !scanner_parent.contains_key(v[1].as_str()) {
                    scanner_parent.insert(v[1].as_str().clone(), Some(v[0].clone()));
                }
                if scanner_parent.contains_key(v[1].as_str())
                    && !scanner_parent.contains_key(v[0].as_str()) {
                    scanner_parent.insert(v[0].as_str().clone(), Some(v[1].clone()));
                }
            });
        }
        let mut all_beacons = HashSet::new();
        for scanner in beacons.keys().sorted() {
            for b in beacons.get(scanner.as_str()).unwrap() {
                let mut b
                    = Vector3::from_iterator(b.0.iter().map(|i| *i as f32));
                let mut s = scanner;
                loop {
                    if let Some(Some(parent)) = scanner_parent.get(s.as_str()) {
                        let pair = [parent.clone(), s.clone()];
                        let alt_pair = [s.clone(), parent.clone()];
                        let pos = scanner_pos.get(pair.as_ref());
                        if pos.is_none() {
                            let pos = scanner_pos.get(alt_pair.as_ref());
                            let pos = &Vector3::from_iterator(
                                pos.unwrap().0.iter().map(|i| *i as f32));
                            let r = *scanner_rotation.get(alt_pair.as_ref()).unwrap();
                            b = rotations[r].inverse().transform_vector(&(b - pos))
                        } else {
                            let pos = &Vector3::from_iterator(
                                pos.unwrap().0.iter().map(|i| *i as f32));
                            let r = *scanner_rotation.get(pair.as_ref()).unwrap();
                            b = pos + rotations[r].transform_vector(&b);
                        }
                        s = parent;
                    } else {
                        break;
                    }
                }

                let pos = Pos([b.x as i64, b.y as i64, b.z as i64 ]);
                all_beacons.insert(pos);
            }
        }
        Ok(all_beacons.len())
    }

    fn part2_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let beacons = Day19::parse(input)?;
        let rotations = &Self::rotations();
        let scanner_pairs = beacons.keys().combinations(2)
            .map(|v| v.into_iter().cloned().sorted().collect_vec())
            .sorted().collect_vec();
        let mut scanner_pos = HashMap::new();
        let mut scanner_rotation = HashMap::new();
        for pair in scanner_pairs {
            let s0_beacons = beacons.get(&pair[0]).unwrap();
            let s1_beacons = beacons.get(&pair[1]).unwrap();
            let mut s0_s1_candidates: HashMap<(usize, Pos), usize>
                = HashMap::new();
            for s0_b in s0_beacons {
                let s0_b = &Vector3::from_iterator(
                    s0_b.0.iter().map(|i| *i as f32));
                for s1_b in s1_beacons {
                    let s1_b = &Vector3::from_iterator(
                        s1_b.0.iter().map(|i| *i as f32));
                    rotations.iter().enumerate().for_each(|(i, r)| {
                        let s0_s1 = s0_b - r.transform_vector(s1_b);
                        let key
                            = (i,
                               Pos([
                                   s0_s1.x as i64,
                                   s0_s1.y as i64,
                                   s0_s1.z as i64
                               ]));
                        s0_s1_candidates.entry(key).and_modify(|e| *e += 1)
                            .or_insert(1);
                    });
                }
            }
            let found = &s0_s1_candidates.iter().filter(|&(_, n)| *n >= 12)
                .collect_vec();
            if !found.is_empty() {
                scanner_rotation.insert(pair.clone(), found[0].0.0.clone());
                scanner_pos.insert(pair.clone(), found[0].0.1.clone());
            }
        }
        let mut scanner_parent = HashMap::new();
        scanner_parent.insert("--- scanner 0 ---", None);
        while beacons.keys().any(|scanner| !scanner_parent.contains_key(scanner.as_str())) {
            scanner_rotation.keys().for_each(|v| {
                if scanner_parent.contains_key(v[0].as_str())
                    && !scanner_parent.contains_key(v[1].as_str()) {
                    scanner_parent.insert(v[1].as_str().clone(), Some(v[0].clone()));
                }
                if scanner_parent.contains_key(v[1].as_str())
                    && !scanner_parent.contains_key(v[0].as_str()) {
                    scanner_parent.insert(v[0].as_str().clone(), Some(v[1].clone()));
                }
            });
        }
        let mut all_scanners = HashSet::new();
        for scanner in beacons.keys().sorted() {
            let mut b = Vector3::new(0.0, 0.0, 0.0);
            let mut s = scanner;
            loop {
                if let Some(Some(parent)) = scanner_parent.get(s.as_str()) {
                    let pair = [parent.clone(), s.clone()];
                    let alt_pair = [s.clone(), parent.clone()];
                    let pos = scanner_pos.get(pair.as_ref());
                    if pos.is_none() {
                        let pos = scanner_pos.get(alt_pair.as_ref());
                        let pos = &Vector3::from_iterator(
                            pos.unwrap().0.iter().map(|i| *i as f32));
                        let r = *scanner_rotation.get(alt_pair.as_ref()).unwrap();
                        b = rotations[r].inverse().transform_vector(&(b - pos))
                    } else {
                        let pos = &Vector3::from_iterator(
                            pos.unwrap().0.iter().map(|i| *i as f32));
                        let r = *scanner_rotation.get(pair.as_ref()).unwrap();
                        b = pos + rotations[r].transform_vector(&b);
                    }
                    s = parent;
                } else {
                    break;
                }
            }
            let pos = Pos([b.x as i64, b.y as i64, b.z as i64]);
            all_scanners.insert(pos);
        }
        all_scanners.iter().combinations(2)
            .map(|x| Self::manhattan(x[0], x[1]) as usize).max()
            .ok_or(AocError.into())
    }

    fn parse(input: &mut dyn Read) -> BoxResult<HashMap<String, BTreeSet<Pos>>> {
        let input = io::BufReader::new(input).lines()
            .collect::<Result<Vec<_>, _>>()?;
        Ok(input.split(|line| line == "").map(|line| {
            let name = line[0].clone();
            let beacons = &line[1..];
            let beacons = beacons.iter().map(|s| Pos::parse(s))
                .collect::<Result<_, _>>();
            beacons.map(|beacons| (name, beacons))
        }).collect::<Result<HashMap<_, _>, _>>()?)
    }

    fn rotations() -> [Rotation<f32, 3>; 24] {
        [
            Rotation::face_towards(&Vector3::x(), &Vector3::y()),
            Rotation::face_towards(&Vector3::x(), &-Vector3::y()),
            Rotation::face_towards(&Vector3::x(), &Vector3::z()),
            Rotation::face_towards(&Vector3::x(), &-Vector3::z()),
            Rotation::face_towards(&-Vector3::x(), &Vector3::y()),
            Rotation::face_towards(&-Vector3::x(), &-Vector3::y()),
            Rotation::face_towards(&-Vector3::x(), &Vector3::z()),
            Rotation::face_towards(&-Vector3::x(), &-Vector3::z()),
            Rotation::face_towards(&Vector3::y(), &Vector3::x()),
            Rotation::face_towards(&Vector3::y(), &-Vector3::x()),
            Rotation::face_towards(&Vector3::y(), &Vector3::z()),
            Rotation::face_towards(&Vector3::y(), &-Vector3::z()),
            Rotation::face_towards(&-Vector3::y(), &Vector3::x()),
            Rotation::face_towards(&-Vector3::y(), &-Vector3::x()),
            Rotation::face_towards(&-Vector3::y(), &Vector3::z()),
            Rotation::face_towards(&-Vector3::y(), &-Vector3::z()),
            Rotation::face_towards(&Vector3::z(), &Vector3::x()),
            Rotation::face_towards(&Vector3::z(), &-Vector3::x()),
            Rotation::face_towards(&Vector3::z(), &Vector3::y()),
            Rotation::face_towards(&Vector3::z(), &-Vector3::y()),
            Rotation::face_towards(&-Vector3::z(), &Vector3::x()),
            Rotation::face_towards(&-Vector3::z(), &-Vector3::x()),
            Rotation::face_towards(&-Vector3::z(), &Vector3::y()),
            Rotation::face_towards(&-Vector3::z(), &-Vector3::y())
        ]
    }

    fn manhattan(a: &Pos, b: &Pos) -> i64 {
        a.0.iter().zip(b.0.iter()).map(|(a, b)| (a - b).abs()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day19 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1("--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14",
              79);
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day19 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2("--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14",
              3621);
    }
}