use std::cmp::Ordering;

use itertools::Itertools;

#[derive(Clone, Copy, Debug)]
enum Token {
    Element(u8),
    StartList,
    EndList
}

struct TokenStream<'a> {
    byte_stream: &'a [u8]
}

impl<'a> TokenStream<'a> {

    fn new(stream: &'a str) -> Self {
        Self {
            byte_stream: stream.as_bytes()
        }
    }

}

impl<'a> Iterator for TokenStream<'a> {

    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.byte_stream[0] == b',' {
            self.byte_stream = &self.byte_stream[1..];
        }

        let token = if self.byte_stream[0] == b'[' {
            self.byte_stream = &self.byte_stream[1..];
            Token::StartList

        } else if self.byte_stream[0] == b']' {
            self.byte_stream = &self.byte_stream[1..];
            Token::EndList

        } else {
            let mut val = 0;
            while self.byte_stream[0] >= b'0' && self.byte_stream[0] <= b'9' {
                val = (val * 10) + ((self.byte_stream[0] - b'0') as u8);
                self.byte_stream = &self.byte_stream[1..];
            }

            Token::Element(val)
        };

        Some(token)
    }

}

fn compare_list(left: &mut impl Iterator<Item = Token>, right: &mut impl Iterator<Item = Token>) -> Ordering {
    loop {
        match (left.next().unwrap(), right.next().unwrap()) {
            (Token::StartList, Token::StartList) => {
                let ord = compare_list(left, right);
                if !ord.is_eq() {
                    return ord;
                }
            },
            (Token::StartList, Token::Element(b)) => {
                let ord = compare_list(left, &mut [Token::Element(b), Token::EndList].into_iter());
                if !ord.is_eq() {
                    return ord;
                }
            },
            (Token::Element(b), Token::StartList) => {
                let ord = compare_list(&mut [Token::Element(b), Token::EndList].into_iter(), right);
                if !ord.is_eq() {
                    return ord;
                }
            },
            (Token::Element(l), Token::Element(r)) => {
                let ord = l.cmp(&r);
                if !ord.is_eq() {
                    return ord;
                }
            },
            (Token::EndList, Token::EndList) => return Ordering::Equal,
            (Token::EndList, _) => return Ordering::Less,
            (_, Token::EndList) => return Ordering::Greater
        }
    }
}

pub fn part1(input: &str) -> u64 {
    let mut score = 0;
    let mut index = 1;
    for mut pair in &input.lines().chunks(3) {
        let left = pair.next().unwrap();
        let right = pair.next().unwrap();
        let ord = compare_list(&mut TokenStream::new(left), &mut TokenStream::new(right));        
        if ord.is_le() {
            score += index;
        }

        index += 1;
    }

    score
}

pub fn part2(input: &str) -> u64 {
    let div_1 = "[[2]]";
    let div_2 = "[[6]]";
    let mut div_1_count = 1;
    let mut div_2_count = 2;
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        {
            let ord = compare_list(&mut TokenStream::new(line), &mut TokenStream::new(div_1));
            if ord.is_le() {
                div_1_count += 1;
            }
        }

        {
            let ord = compare_list(&mut TokenStream::new(line), &mut TokenStream::new(div_2));
            if ord.is_le() {
                div_2_count += 1;
            }
        }
    }

    div_1_count * div_2_count
}
