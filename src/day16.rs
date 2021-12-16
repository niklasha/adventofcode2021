use crate::day::*;
use std::str::Chars;
use itertools::FoldWhile::{Done, Continue};

pub struct Day16 {}

type Output = i64;

impl Day for Day16 {
    fn tag(&self) -> &str { "16" }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

impl Day16 {
    fn part1_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<Output> {
        Ok(Self::packet(&mut Self::bits(input)?.chars())?.1)
    }

    fn part2_impl(self: &Self, input: &mut dyn io::Read) -> BoxResult<Output> {
        Ok(Self::packet(&mut Self::bits(input)?.chars())?.2)
    }

    fn bits(input: &mut dyn io::Read) -> BoxResult<String> {
        Ok(io::BufReader::new(input).lines().next()
            .ok_or(AocError)??.chars()
            .map(|c|
                u8::from_str_radix(c.to_string().as_str(), 16)
                    .map(|d| format!("{:04b}", d)))
            .collect::<Result<Vec<_>, _>>()?.join(""))
    }

    fn packet(bits: &mut Chars) -> BoxResult<(usize, Output, Output)> {
        let version = Output::from_str_radix(
            bits.take(3).collect::<String>().as_str(), 2)?;
        let type_ = i64::from_str_radix(
            bits.take(3).collect::<String>().as_str(), 2)?;
        let (len, sum, value) = match type_ {
            4 => Self::literal(bits)?,
            _ => Self::operator(type_, bits)?
        };
        Ok((6 + len, version + sum, value))
    }

    fn literal(bits: &mut Chars) -> BoxResult<(usize, Output, Output)> {
        let chunks = bits.chunks(5);
        let (len, value) = chunks.into_iter()
            .fold_while(
                Ok((0, 0)),
                |len_val, mut bits| match len_val {
                    Ok((len, value)) => match bits.next() {
                        Some(bit) => {
                            let rv
                                = Output::from_str_radix(
                                &bits.take(4).collect::<String>(), 2)
                                .map(|number| (5 + len, value * 16 + number))
                                .map_err(|e| AocError);
                            if bit == '1' { Continue(rv) } else { Done(rv) }
                        },
                        _ => Done(Err(AocError))
                    },
                    _ => Done(len_val)
                })
            .into_inner()?;
        Ok((len, 0, value))
    }

    fn operator(op: i64, bits: &mut Chars)
        -> BoxResult<(usize, Output, Output)> {
        let id = bits.next().ok_or(AocError)?;
        let (len, sum, args) = match id {
            '0' => {
                let len = usize::from_str_radix(
                    bits.take(15).collect::<String>().as_str(), 2)?;
                let (len, sum, args) = (0..).fold_while(
                    (0, 0, vec![]),
                    |(total_len, total_sum, mut args), _| {
                        let (inner_len, sum, value) = Self::packet(bits).unwrap();
                        args.push(value);
                        let rv = (total_len + inner_len, total_sum + sum, args);
                        if rv.0 >= len { Done(rv) } else { Continue(rv) }
                    }).into_inner();
                (16 + len, sum, args)
            },
            '1' => {
                let count = usize::from_str_radix(
                    bits.take(11).collect::<String>().as_str(), 2)?;
                (0..count)
                    .fold(
                        (12, 0, vec![]), |(total_len, total_sum, mut args), _| {
                            let (len, sum, value) = Self::packet(bits).unwrap();
                            args.push(value);
                            (total_len + len, total_sum + sum, args)
                        })
            },
            _ => Err(AocError)?
        };
        let value = match op {
            0 => args.into_iter().sum::<Output>(),
            1 => args.into_iter().product(),
            2 => args.into_iter().min().unwrap(),
            3 => args.into_iter().max().unwrap(),
            5 => if args[0] > args[1] { 1 } else { 0 },
            6 => if args[0] < args[1] { 1 } else { 0 },
            7 => if args[0] == args[1] { 1 } else { 0 },
            _ => 0 // XXX error
        };
        Ok((len, sum, value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day16 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1("8A004A801A8002F478", 16);
        test1("620080001611562C8802118E34", 12);
        test1("C0015000016115A2E0802F182340", 23);
        test1("A0016C880162017C3686B18A3D4780", 31);
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day16 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2("C200B40A82", 3);
        test2("04005AC33890", 54);
        test2("880086C3E88112", 7);
        test2("CE00C43D881120", 9);
        test2("D8005AC2A8F0", 1);
        test2("F600BC2D8F", 0);
        test2("9C005AC2F8F0", 0);
        test2("9C0141080250320F1802104A08", 1);
    }
}