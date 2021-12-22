use crate::day::*;
use regex::Regex;
use std::io::Read;
use std::collections::HashSet;
use std::{fmt, cmp, iter};
use std::ops::{Add, Mul, Sub};

pub struct Day22 {}

type Output = i64;

impl Day for Day22 {
    fn tag(&self) -> &str { "22" }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Cuboid {
    x0: i64,
    y0: i64,
    z0: i64,
    x1: i64,
    y1: i64,
    z1: i64
}

impl Cuboid {
    fn new(x0: i64, x1: i64, y0: i64, y1: i64, z0: i64, z1: i64) -> Self {
        Self {
            // XXX Canonicalization can not happen here due to an impementation
            // XXX detail of the "split" function.
            // x0: cmp::min(x0, x1),
            // x1: cmp::max(x0, x1),
            // y0: cmp::min(y0, y1),
            // y1: cmp::max(y0, y1),
            // z0: cmp::min(z0, z1),
            // z1: cmp::max(z0, z1)
            x0, x1, y0, y1, z0, z1
        }
    }

    fn abs(&self) -> i64 {
        (self.x1 - self.x0 + 1) * (self.y1 - self.y0 + 1)
            * (self.z1 - self.z0 + 1)
    }

    fn split(&self, other : &Self) -> HashSet<Self> {
        let (min_x0, max_x0, min_x1, max_x1, min_y0, max_y0, min_y1, max_y1,
            min_z0, max_z0, min_z1, max_z1)
            = (cmp::min(self.x0, other.x0), cmp::max(self.x0, other.x0),
               cmp::min(self.x1, other.x1), cmp::max(self.x1, other.x1),
               cmp::min(self.y0, other.y0), cmp::max(self.y0, other.y0),
               cmp::min(self.y1, other.y1), cmp::max(self.y1, other.y1),
               cmp::min(self.z0, other.z0), cmp::max(self.z0, other.z0),
               cmp::min(self.z1, other.z1), cmp::max(self.z1, other.z1));
        [
            Self::new(min_x0, max_x0 - 1, min_y0, max_y0 - 1, min_z0, max_z0 - 1),
            Self::new(min_x0, max_x0 - 1, min_y0, max_y0 - 1, max_z0, min_z1),
            Self::new(min_x0, max_x0 - 1, min_y0, max_y0 - 1, min_z1 + 1 , max_z1),
            Self::new(min_x0, max_x0 - 1, max_y0, min_y1, min_z0, max_z0 - 1),
            Self::new(min_x0, max_x0 - 1, max_y0, min_y1, max_z0, min_z1),
            Self::new(min_x0, max_x0 - 1, max_y0, min_y1, min_z1 + 1, max_z1),
            Self::new(min_x0, max_x0 - 1, min_y1 + 1, max_y1, min_z0, max_z0 - 1),
            Self::new(min_x0, max_x0 - 1, min_y1 + 1, max_y1, max_z0, min_z1),
            Self::new(min_x0, max_x0 - 1, min_y1 + 1, max_y1, min_z1 + 1, max_z1),

            Self::new(max_x0, min_x1, min_y0, max_y0 - 1, min_z0, max_z0 - 1),
            Self::new(max_x0, min_x1, min_y0, max_y0 - 1, max_z0, min_z1),
            Self::new(max_x0, min_x1, min_y0, max_y0 - 1, min_z1 + 1, max_z1),
            Self::new(max_x0, min_x1, max_y0, min_y1, min_z0, max_z0 - 1),
            Self::new(max_x0, min_x1, max_y0, min_y1, max_z0, min_z1),
            Self::new(max_x0, min_x1, max_y0, min_y1, min_z1 + 1, max_z1),
            Self::new(max_x0, min_x1, min_y1 + 1, max_y1, min_z0, max_z0 - 1),
            Self::new(max_x0, min_x1, min_y1 + 1, max_y1, max_z0, min_z1),
            Self::new(max_x0, min_x1, min_y1 + 1, max_y1, min_z1 + 1, max_z1),

            Self::new(min_x1 + 1, max_x1, min_y0, max_y0 - 1, min_z0, max_z0 - 1),
            Self::new(min_x1 + 1, max_x1, min_y0, max_y0 - 1, max_z0, min_z1),
            Self::new(min_x1 + 1, max_x1, min_y0, max_y0 - 1, min_z1 + 1, max_z1),
            Self::new(min_x1 + 1, max_x1, max_y0, min_y1, min_z0, max_z0 - 1),
            Self::new(min_x1 + 1, max_x1, max_y0, min_y1, max_z0, min_z1),
            Self::new(min_x1 + 1, max_x1, max_y0, min_y1, min_z1 + 1, max_z1),
            Self::new(min_x1 + 1, max_x1, min_y1 + 1, max_y1, min_z0, max_z0 - 1),
            Self::new(min_x1 + 1, max_x1, min_y1 + 1, max_y1, max_z0, min_z1),
            Self::new(min_x1 + 1, max_x1, min_y1 + 1, max_y1, min_z1 + 1, max_z1),
        ].iter()
            .filter(|Cuboid { x0, x1, y0, y1, z0, z1 }|
                x0 <= x1 && y0 <= y1 && z0 <= z1)
            // XXX opportunity for coalescing here?
            .map(|cuboid| *cuboid)
            .collect()
    }

    fn contains(&self, other: &Self) -> bool {
        self.x0 <= other.x0 && self.x1 >= other.x1 && self.y0 <= other.y0
            && self.y1 >= other.y1 && self.z0 <= other.z0 && self.z1 >= other.z1
    }

    fn print(&self) {
        for x in self.x0..=self.x1 {
            for y in self.y0..=self.y1 {
                for z in self.z0..=self.z1 {
                    println!("  {},{},{}", x, y, z);
                }
            }
        }
    }
}

impl Add for Cuboid {
    type Output = HashSet<Cuboid>;

    fn add(self, rhs: Self) -> Self::Output {
        let (x0, x1, y0, y1, z0, z1)
            = (cmp::max(self.x0, rhs.x0),
               cmp::min(self.x1, rhs.x1),
               cmp::max(self.y0, rhs.y0),
               cmp::min(self.y1, rhs.y1),
               cmp::max(self.z0, rhs.z0),
               cmp::min(self.z1, rhs.z1));
        if x0 >= x1 || y0 >= y1 || z0 >= z1 {
            [self, rhs].iter().map(|cuboid| *cuboid).collect()
        } else { self.split(&rhs) }
    }
}

impl Sub for Cuboid {
    type Output = HashSet<Cuboid>;

    fn sub(self, rhs: Self) -> HashSet<Self> {
        let (x0, x1, y0, y1, z0, z1)
            = (cmp::max(self.x0, rhs.x0),
               cmp::min(self.x1, rhs.x1),
               cmp::max(self.y0, rhs.y0),
               cmp::min(self.y1, rhs.y1),
               cmp::max(self.z0, rhs.z0),
               cmp::min(self.z1, rhs.z1));
        if x0 > x1 || y0 > y1 || z0 > z1 { iter::once(self).collect() }
        else {
            let split = self.split(&rhs);
            let rv = split.iter()
                .filter(|cuboid| !rhs.contains(&cuboid))
                .flat_map(|cuboid| self * *cuboid).collect::<HashSet<_>>();
            rv
        }
    }
}

impl Mul for Cuboid {
    type Output = Option<Cuboid>;

    fn mul(self, rhs: Self) -> Self::Output {
        let (x0, x1, y0, y1, z0, z1)
            = (cmp::max(self.x0, rhs.x0),
               cmp::min(self.x1, rhs.x1),
               cmp::max(self.y0, rhs.y0),
               cmp::min(self.y1, rhs.y1),
               cmp::max(self.z0, rhs.z0),
               cmp::min(self.z1, rhs.z1));
        if x0 > x1 || y0 > y1 || z0 > z1 { None }
        else { Some(Self::new(x0, x1, y0, y1, z0, z1)) }
    }
}

impl fmt::Display for Cuboid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, "x={}..{},y={}..{},z={}..{}", self.x0, self.x1, self.y0, self.y1,
            self.z0, self.z1)
    }
}

impl fmt::Debug for Cuboid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Day22 {
    fn part1_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<Output> {
        Self::process(input, Some(Cuboid::new(-50, 50, -50, 50, -50, 50)))
    }

    fn part2_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<Output> {
        Self::process(input, None)
    }

    fn process(input: &mut dyn io::Read, bounding_box: Option<Cuboid>)
               -> BoxResult<Output> {
        let steps = Self::parse(input)?;
        let core: HashSet<Cuboid> = steps.iter().fold(HashSet::new(), |core, step| {
            match step {
                (true, a) =>
                    core.iter().flat_map(|b| *b - *a).chain(iter::once(*a))
                        .collect(),
                (false, a) => core.iter().flat_map(|b| *b - *a).collect()
            }
        });
        if let Some(bounding_box) = bounding_box {
            Ok(core.iter()
                .flat_map(|cuboid| *cuboid * bounding_box)
                .map(|cuboid| cuboid.abs()).sum())
        } else {
            Ok(core.iter().map(|cuboid| cuboid.abs()).sum())
        }

    }

    fn parse(input: &mut dyn Read) -> BoxResult<Vec<(bool, Cuboid)>> {
        let mut lines = io::BufReader::new(input).lines()
            .collect::<Result<Vec<_>, _>>()?;
        lines.iter().map(|line| Self::parse_step(line))
            .collect::<Result<Vec<_>, _>>().into()
    }

    fn parse_step(s: &str) -> BoxResult<(bool, Cuboid)> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                "(on|off) x=(-?\\d+)\\.\\.(-?\\d+),y=(-?\\d+)\\.\\.(-?\\d+),z=(-?\\d+)\\.\\.(-?\\d+)")
            .unwrap();
        }
        let cap = RE.captures(&s).ok_or(AocError)?;
        Ok((&cap[1] == "on",
            Cuboid::new(
                cap[2].parse()?, cap[3].parse()?,
                cap[4].parse()?, cap[5].parse()?,
                cap[6].parse()?, cap[7].parse()?)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day22 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1("on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10",
              39);
        test1("on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682",
              590784);
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day22 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2("on x=-5..47,y=-31..22,z=-19..33
on x=-44..5,y=-27..21,z=-14..35
on x=-49..-1,y=-11..42,z=-10..38
on x=-20..34,y=-40..6,z=-44..1
off x=26..39,y=40..50,z=-2..11
on x=-41..5,y=-41..6,z=-36..8
off x=-43..-33,y=-45..-28,z=7..25
on x=-33..15,y=-32..19,z=-34..11
off x=35..47,y=-46..-34,z=-11..5
on x=-14..36,y=-6..44,z=-16..29
on x=-57795..-6158,y=29564..72030,z=20435..90618
on x=36731..105352,y=-21140..28532,z=16094..90401
on x=30999..107136,y=-53464..15513,z=8553..71215
on x=13528..83982,y=-99403..-27377,z=-24141..23996
on x=-72682..-12347,y=18159..111354,z=7391..80950
on x=-1060..80757,y=-65301..-20884,z=-103788..-16709
on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856
on x=-52752..22273,y=-49450..9096,z=54442..119054
on x=-29982..40483,y=-108474..-28371,z=-24328..38471
on x=-4958..62750,y=40422..118853,z=-7672..65583
on x=55694..108686,y=-43367..46958,z=-26781..48729
on x=-98497..-18186,y=-63569..3412,z=1232..88485
on x=-726..56291,y=-62629..13224,z=18033..85226
on x=-110886..-34664,y=-81338..-8658,z=8914..63723
on x=-55829..24974,y=-16897..54165,z=-121762..-28058
on x=-65152..-11147,y=22489..91432,z=-58782..1780
on x=-120100..-32970,y=-46592..27473,z=-11695..61039
on x=-18631..37533,y=-124565..-50804,z=-35667..28308
on x=-57817..18248,y=49321..117703,z=5745..55881
on x=14781..98692,y=-1341..70827,z=15753..70151
on x=-34419..55919,y=-19626..40991,z=39015..114138
on x=-60785..11593,y=-56135..2999,z=-95368..-26915
on x=-32178..58085,y=17647..101866,z=-91405..-8878
on x=-53655..12091,y=50097..105568,z=-75335..-4862
on x=-111166..-40997,y=-71714..2688,z=5609..50954
on x=-16602..70118,y=-98693..-44401,z=5197..76897
on x=16383..101554,y=4615..83635,z=-44907..18747
off x=-95822..-15171,y=-19987..48940,z=10804..104439
on x=-89813..-14614,y=16069..88491,z=-3297..45228
on x=41075..99376,y=-20427..49978,z=-52012..13762
on x=-21330..50085,y=-17944..62733,z=-112280..-30197
on x=-16478..35915,y=36008..118594,z=-7885..47086
off x=-98156..-27851,y=-49952..43171,z=-99005..-8456
off x=2032..69770,y=-71013..4824,z=7471..94418
on x=43670..120875,y=-42068..12382,z=-24787..38892
off x=37514..111226,y=-45862..25743,z=-16714..54663
off x=25699..97951,y=-30668..59918,z=-15349..69697
off x=-44271..17935,y=-9516..60759,z=49131..112598
on x=-61695..-5813,y=40978..94975,z=8655..80240
off x=-101086..-9439,y=-7088..67543,z=33935..83858
off x=18020..114017,y=-48931..32606,z=21474..89843
off x=-77139..10506,y=-89994..-18797,z=-80..59318
off x=8476..79288,y=-75520..11602,z=-96624..-24783
on x=-47488..-1262,y=24338..100707,z=16292..72967
off x=-84341..13987,y=2429..92914,z=-90671..-1318
off x=-37810..49457,y=-71013..-7894,z=-105357..-13188
off x=-27365..46395,y=31009..98017,z=15428..76570
off x=-70369..-16548,y=22648..78696,z=-1892..86821
on x=-53470..21291,y=-120233..-33476,z=-44150..38147
off x=-93533..-4276,y=-16170..68771,z=-104985..-24507",
              2758514936282235);
    }
}