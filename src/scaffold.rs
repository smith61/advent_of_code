
use std::{fmt::Display, path::Path, sync::Arc};

use reqwest::{blocking::ClientBuilder, cookie::Jar, Url};

use crate::utils::{Matrix2DBorrowed, Vector2};

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
    String(String),
    Vector2(Vector2)
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

    pub fn next_vector2(&mut self) -> Option<Vector2> {
        Some(Vector2::new(self.next_int()?, self.next_int()?))
    }

    pub fn next_vector2s<const COUNT: usize>(&mut self) -> Option<[Vector2; COUNT]> {
        let mut values = [Vector2::default(); COUNT];
        for index in 0..COUNT {
            values[index] = self.next_vector2()?;
        }

        Some(values)
    }

}

impl<'a> From<InputParser<'a>> for &'a str {

    fn from(value: InputParser<'a>) -> Self {
        std::str::from_utf8(value.input_str).unwrap()
    }

}

impl<'a> From<InputParser<'a>> for Matrix2DBorrowed<'a, u8> {

    fn from(value: InputParser<'a>) -> Self {
        Self::from_input_lines(std::str::from_utf8(value.input_str).unwrap())
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

impl From<Vector2> for AocResult {

    fn from(inner: Vector2) -> Self {
        AocResult::Vector2(inner)
    }

}

impl Display for AocResult {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AocResult::I64(val) => write!(f, "{}", val),
            AocResult::U64(val) => write!(f, "{}", val),
            AocResult::String(val) => write!(f, "{}", val),
            AocResult::Vector2(val) => write!(f, "{},{}", val.x(), val.y())
        }
    }

}

fn download_input(aoc_year: &str, aoc_day: &str, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let year_num = &aoc_year["year_".len()..];
    let day_num = if aoc_day.starts_with("day_0") {
        &aoc_day["day_0".len()..]

    } else {
        &aoc_day["day_".len()..]
    };

    let mut session_token_file = std::env::current_dir()?;
    session_token_file.push("input");
    session_token_file.push("session-token.txt");
    if !session_token_file.is_file() {
        panic!("Failed to find session token file");
    }

    let session_token = std::fs::read(session_token_file)?;
    let session_token = std::str::from_utf8(&session_token)?;

    let input_url = Url::parse(&format!("https://adventofcode.com/{}/day/{}/input", year_num, day_num))?;
    let client = {
        let cookie_store = Jar::default();
        cookie_store.add_cookie_str(&format!("session={}", session_token), &input_url);

        ClientBuilder::new()
            .cookie_provider(Arc::new(cookie_store))
            .build()?
    };

    let response = client.get(input_url).send()?;

    std::fs::create_dir_all(path.parent().unwrap())?;
    std::fs::write(path, response.text()?)?;

    Ok(())
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

    if !example_input && !path.is_file() {
        println!("Input not found for {}-{}, attempting to download it...", aoc_year, aoc_day);
        if let Err(error) = download_input(aoc_year, aoc_day, &path) {
            panic!("Failed to download input file: {:?}", error);
        }

        println!("Successfully downloaded input file");
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

        $(pub(crate) mod $year {
            $(pub(crate) mod $day;)*

            pub(crate) mod bench {
                $(
                    pub(crate) mod $day {

                        use criterion::Criterion;

                        pub(crate) fn run_bench(c: &mut Criterion) {
                            let input = $crate::scaffold::get_input(stringify!($year), stringify!($day), false);

                            c.bench_function(concat!(stringify!($year), "::", stringify!($day), "::part_1"), |b| b.iter(|| $crate::$year::$day::part1($crate::scaffold::InputParser::new(&input).into())));
                            c.bench_function(concat!(stringify!($year), "::", stringify!($day), "::part_2"), |b| b.iter(|| $crate::$year::$day::part2($crate::scaffold::InputParser::new(&input).into())));
                        }
                    }
                )*
            }
        })*

        pub mod bench {
            $(criterion::criterion_group!($year, $($crate::$year::bench::$day::run_bench),*);)*
        }

        pub fn aoc_bin_main() {
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
