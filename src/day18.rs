use crate::day::*;
use std::fmt;

pub struct Day18 {}

type Output = i64;

impl Day for Day18 {
    fn tag(&self) -> &str { "18" }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

#[derive(Clone)]
enum Node {
    Pair(Box<Node>, Box<Node>),
    Regular(Output)
}

impl Node {
    fn new_pair(left: Self, right: Self) -> Self {
        Self::Pair(Box::new(left), Box::new(right))
    }

    fn new_regular(number: Output) -> Self { Self::Regular(number) }

    fn from(s: &str) -> BoxResult<Self> {
        let mut stack = vec![];
        for c in s.chars() {
            match c {
                '[' | ',' => {}
                ']' => {
                    let (b, a)
                        = (stack.pop().ok_or(AocError)?,
                            stack.pop().ok_or(AocError)?);
                    stack.push(Self::new_pair(a, b));
                }
                '0'..='9' => stack.push(Self::new_regular(c as Output - '0' as Output)),
                _ => Err(AocError)?
            }
        }
        Ok(stack.pop().ok_or(AocError)?)
    }

    fn is_pair(&self) -> bool {
        match self { Self::Pair(_, _) => true, _ => false }
    }

    fn is_regular(&self) -> bool {
        match self { Self::Regular(_) => true, _ => false }
    }

    fn pair(&self) -> Option<(&Box<Self>, &Box<Self>)> {
        match self { Self::Pair(l, r) => Some((l, r)), _ => None }
    }

    fn regular(&self) -> Option<Output> {
        match self { Self::Regular(n) => Some(*n), _ => None }
    }

    fn add(self, n: Node) -> BoxResult<Node> {
        let mut sum = Self::new_pair(self, n);
        sum.reduce()?;
        Ok(sum)
    }

    fn magnitude(&self) -> Output {
        match self {
            Self::Regular(n) => *n,
            Self::Pair(l, r) => 3 * l.magnitude() + 2 * r.magnitude()
        }
    }

    fn reduce(&mut self) -> BoxResult<()> {
        loop {
            loop {
                let (mut to_the_left, mut to_the_right) = (None, None);
                if !self.explode(
                    &mut to_the_left, &mut to_the_right, 0, true)? {
                    break
                }
                self.remove_exploded_pair(0);
            }
            if !self.split() { break Ok(()); }
        }
    }

    fn remove_exploded_pair(&mut self, depth: usize) -> bool {
        match self {
            Self::Pair(l, r) => if depth == 4 {
                *self = Self::new_regular(0);
                true
            } else {
                l.remove_exploded_pair(depth + 1) || r.remove_exploded_pair(depth + 1)
            },
            _ => false
        }
    }

    fn explode<'a>(
        &'a mut self, to_the_left: &mut Option<&'a mut Self>,
        to_the_right: &mut Option<Output>, depth: usize, armed: bool)
        -> BoxResult<bool> {
        match self {
            Self::Regular(_) => {
                if let Some(n) = to_the_right {
                    *self = Self::new_regular(
                        self.regular().ok_or(AocError)? + *n);
                    *to_the_right = None;
                } else {
                    *to_the_left = Some(self);
                }
                Ok(false)
            },
            Self::Pair(left, right) =>
                if depth == 4 && armed {
                    if let Some(mut lhs) = to_the_left.as_mut() {
                        **lhs = Self::new_regular(
                            lhs.regular().ok_or(AocError)?
                                + left.regular().ok_or(AocError)?);
                    }
                    *to_the_right = Some(right.regular().ok_or(AocError)?);
                    // Due to borrowing rules, I could not replace the current
                    // node with a regular number zero here.  Instead I patch
                    // that up in a separate function: remove_exploded_pair.
                    Ok(true)
                } else {
                    let did_explode = left.explode(
                        to_the_left, to_the_right, depth + 1, armed)?;
                    Ok(did_explode
                           | right.explode(
                        to_the_left, to_the_right, depth + 1,
                        armed && !did_explode)?)
                }
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Self::Regular(n) => {
                *n >= 10 && {
                    *self = Self::new_pair(
                        Self::new_regular(*n / 2),
                        Self::new_regular(*n / 2 + *n % 2));
                    true
                }
            },
            Self::Pair(l, r) => l.split() || r.split()
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Regular(n) => write!(f, "{}", n),
            Self::Pair(l, r) => write!(f, "[{},{}]", l, r)
        }
    }
}

impl Day18 {
    fn part1_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let nodes = io::BufReader::new(input).lines().map(|line| {
            line.map_err(Into::into).and_then(|line| Node::from(line.as_str()))
        }).collect::<Result<Vec<_>, _>>()?;
        Ok(nodes.into_iter().reduce(|a, b| a.add(b).unwrap()).ok_or(AocError)?
            .magnitude())
    }

    fn part2_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let nodes = io::BufReader::new(input).lines().map(|line| {
            line.map_err(Into::into).and_then(|line| Node::from(line.as_str()))
        }).collect::<Result<Vec<_>, _>>()?;
        nodes.iter().permutations(2)
            .map(|perm|
                perm[0].clone().add(perm[1].clone()).unwrap().magnitude())
            .max().ok_or(AocError.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day18 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        for number in [
            "[1,2]",
            "[[1,2],3]",
            "[9,[8,7]]",
            "[[1,9],[8,5]]",
            "[[[[1,2],[3,4]],[[5,6],[7,8]]],9]",
            "[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]",
            "[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]"
        ] {
            assert_eq!(number, format!("{}", Node::from(number).unwrap()));
        }
        test1("[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
              4140);
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day18 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2("[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
              3993);
    }
}