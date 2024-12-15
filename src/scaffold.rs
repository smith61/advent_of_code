
use std::fmt::Display;

use crate::utils::{Grid2DBorrowed, Point2D};

pub struct AocYear {
    pub year: &'static str,
    pub days: &'static [AocDay]
}

pub struct AocDay {
    pub day: &'static str,
    pub part_1: fn(InputParser) -> AocResult,
    pub part_2: fn(InputParser) -> AocResult
}

pub enum AocResult {
    I64(i64),
    U64(u64),
    String(String)
}

#[derive(Clone, Copy)]
pub struct InputParser<'a> {
    input_str: &'a [u8]
}

impl<'a> InputParser<'a> {

    pub fn new(input_str: &'a str) -> Self {
        Self {
            input_str: input_str.as_bytes()
        }
    }
    
    pub fn next_int(&mut self) -> Option<isize> {
        loop {
            if self.input_str.is_empty() {
                return None;
            }

            let mut sign = 1;
            if self.input_str[0] == b'-' {
                sign = -1;
                self.input_str = &self.input_str[1..];
                if self.input_str.is_empty() {
                    return None;
                }
            }

            if !self.input_str[0].is_ascii_digit() {
                self.input_str = &self.input_str[1..];
                continue;
            }

            let mut value = 0;
            while !self.input_str.is_empty() &&
                  self.input_str[0].is_ascii_digit() {

                value = (value * 10) + ((self.input_str[0] - b'0') as isize);
                self.input_str = &self.input_str[1..];
            }

            return Some(value * sign);
        }
    }

    pub fn next_ints<const COUNT: usize>(&mut self) -> Option<[isize; COUNT]> {
        let mut values = [0; COUNT];
        for index in 0..COUNT {
            values[index] = self.next_int()?;
        }

        Some(values)
    }

    pub fn next_uint(&mut self) -> Option<u64> {
        while !self.input_str.is_empty() &&
              !self.input_str[0].is_ascii_digit() {

            self.input_str = &self.input_str[1..];
        }

        if !self.input_str.is_empty() {
            let mut value = 0;
            while !self.input_str.is_empty() &&
                  self.input_str[0].is_ascii_digit() {

                value = (value * 10) + ((self.input_str[0] - b'0') as u64);
                self.input_str = &self.input_str[1..];
            }

            Some(value)

        } else {
            None
        }
    }

    pub fn next_uints<const COUNT: usize>(&mut self) -> Option<[u64; COUNT]> {
        let mut values = [0; COUNT];
        for index in 0..COUNT {
            values[index] = self.next_uint()?;
        }

        Some(values)
    }

    pub fn next_point2d(&mut self) -> Option<Point2D> {
        Some(Point2D::new(self.next_int()?, self.next_int()?))
    }

    pub fn next_point2ds<const COUNT: usize>(&mut self) -> Option<[Point2D; COUNT]> {
        let mut values = [Point2D::default(); COUNT];
        for index in 0..COUNT {
            values[index] = self.next_point2d()?;
        }

        Some(values)
    }

}

impl<'a> From<InputParser<'a>> for &'a str {

    fn from(value: InputParser<'a>) -> Self {
        std::str::from_utf8(value.input_str).unwrap()
    }

}

impl<'a> From<InputParser<'a>> for Grid2DBorrowed<'a> {

    fn from(value: InputParser<'a>) -> Self {
        Grid2DBorrowed::from_input_lines(std::str::from_utf8(value.input_str).unwrap())
    }

}

impl From<i64> for AocResult {

    fn from(inner: i64) -> Self {
        AocResult::I64(inner)
    }

}

impl From<u64> for AocResult {

    fn from(inner: u64) -> Self {
        AocResult::U64(inner)
    }

}

impl From<String> for AocResult {

    fn from(inner: String) -> Self {
        AocResult::String(inner)
    }

}

impl Display for AocResult {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AocResult::I64(val) => write!(f, "{}", val),
            AocResult::U64(val) => write!(f, "{}", val),
            AocResult::String(val) => write!(f, "{}", val)
        }
    }

}

pub fn get_input(aoc_year: &str, aoc_day: &str, example_input: bool) -> String {
    let mut path = std::env::current_dir().unwrap();
    path.push("input");
    path.push(aoc_year);
    path.push(aoc_day);
    if example_input {
        path.push("example.txt");

    } else {
        path.push("input.txt");
    }

    if !path.is_file() {
        panic!("Failed to find input file at {:?}", path);
    }

    std::fs::read_to_string(path).unwrap()
}

#[macro_export]
macro_rules! aoc_solvers {
    {
        $($year:ident {
            $($day:ident),*
        }),*
    } => {
        $(mod $year {
            $(pub mod $day;)*

            #[cfg(test)]
            mod __bench {

                $(
                    mod $day {

                        use test::Bencher;

                        #[bench]
                        fn bench_part_1(b: &mut Bencher) {
                            let input = $crate::scaffold::get_input(stringify!($year), stringify!($day), false);
                            b.iter(|| std::hint::black_box($crate::$year::$day::part1($crate::scaffold::InputParser::new(&input).into())));
                        }

                        #[bench]
                        fn bench_part_2(b: &mut Bencher) {
                            let input = $crate::scaffold::get_input(stringify!($year), stringify!($day), false);
                            b.iter(|| std::hint::black_box($crate::$year::$day::part2($crate::scaffold::InputParser::new(&input).into())));
                        }
                    }
                )*
            }
        })*

        fn main() {
            const AOC_YEARS: &[$crate::scaffold::AocYear] = &[
                $(
                    $crate::scaffold::AocYear {
                        year: stringify!($year),
                        days: &[
                            $(
                                $crate::scaffold::AocDay {
                                    day: stringify!($day),
                                    part_1: {
                                        fn wrapper(input: $crate::scaffold::InputParser) -> AocResult {
                                            $crate::scaffold::AocResult::from($crate::$year::$day::part1(input.into()))
                                        }

                                        wrapper
                                    },
                                    part_2: {
                                        fn wrapper(input: $crate::scaffold::InputParser) -> AocResult {
                                            $crate::scaffold::AocResult::from($crate::$year::$day::part2(input.into()))
                                        }

                                        wrapper
                                    }
                                }
                            ),*
                        ]
                    }
                ),*
            ];

            aoc_main(AOC_YEARS);
        }


    };
}
