use crate::day::*;

pub struct Day10 {}

type Output = i64;

impl Day for Day10 {
    fn tag(&self) -> &str { "10" }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

impl Day10 {
    fn part1_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let inputs = io::BufReader::new(input).lines()
            .map(|l| l.unwrap().chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut st = vec![];
        Ok(inputs.into_iter().map(|input| input.into_iter().fold(0, |sc, ch| {
            if sc != 0 { sc } else {
                match ch {
                    '(' | '[' | '{' | '<' => { st.push(ch); sc },
                    ')' => if st.pop().unwrap() == '(' { 0 } else { 3 },
                    ']' => if st.pop().unwrap() == '[' { 0 } else { 57 },
                    '}' => if st.pop().unwrap() == '{' { 0 } else { 1197 },
                    '>' => if st.pop().unwrap() == '<' { 0 } else { 25137 },
                    _ => sc
                }
            }
        })).sum())
    }

    fn part2_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let inputs = io::BufReader::new(input).lines()
            .map(|l| l.unwrap().chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut scores = inputs.iter().flat_map(|input| {
            let mut st = vec![];
            if input.into_iter().fold(0, |sc, &ch| {
                if sc != 0 { sc } else {
                    match ch {
                        '(' | '[' | '{' | '<' => {
                            st.push(ch);
                            sc
                        },
                        ')' => if st.pop().unwrap() == '(' { 0 } else { 3 },
                        ']' => if st.pop().unwrap() == '[' { 0 } else { 57 },
                        '}' => if st.pop().unwrap() == '{' { 0 } else { 1197 },
                        '>' => if st.pop().unwrap() == '<' { 0 } else { 25137 },
                        _ => sc
                    }
                }
            }) == 0 {
                st.reverse();
                Some(st.iter().fold(0, |sc, token| sc * 5 + match token {
                    '(' => 1, '[' => 2, '{' => 3, '<' => 4, _ => 0
                }))
            } else { None }
        }).collect::<Vec<_>>();
        scores.sort();
        Ok(scores[scores.len() / 2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day10 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1(
            "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]",
            26397);
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day10 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2(
            "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]",
            288957);
    }
}