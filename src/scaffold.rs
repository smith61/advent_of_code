
use std::fmt::Display;

pub struct AocYear {
    pub year: &'static str,
    pub days: &'static [AocDay]
}

pub struct AocDay {
    pub day: &'static str,
    pub part_1: fn(&str) -> AocResult,
    pub part_2: fn(&str) -> AocResult
}

pub enum AocResult {
    I64(i64),
    U64(u64),
    String(String)
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
                            b.iter(|| std::hint::black_box($crate::$year::$day::part1(&input)));
                        }

                        #[bench]
                        fn bench_part_2(b: &mut Bencher) {
                            let input = $crate::scaffold::get_input(stringify!($year), stringify!($day), false);
                            b.iter(|| std::hint::black_box($crate::$year::$day::part2(&input)));
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
                                        fn wrapper(input: &str) -> AocResult {
                                            $crate::scaffold::AocResult::from($crate::$year::$day::part1(input))
                                        }

                                        wrapper
                                    },
                                    part_2: {
                                        fn wrapper(input: &str) -> AocResult {
                                            $crate::scaffold::AocResult::from($crate::$year::$day::part2(input))
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
