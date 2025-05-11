#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Position {
    start: (usize, usize), // (row, col)
    end:   (usize, usize),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Token {
    Identifier(Position, String),  // [A-Za-z_][A-Za-z0-9_]*
    NumLiteral(Position, String),  // [0-9]+
    StrLiteral(Position, String),  // '"'[...]'"' TODO: figure this out 
    LParen(Position),              // (
    RParen(Position),              // )
    LBrace(Position),              // {
    RBrace(Position),              // }
    Arrow(Position),               // ->
    FatArrow(Position),            // =>
    Eq(Position),                  // =
    EqEq(Position),                // ==
    Lt(Position),                  // <
    Gt(Position),                  // >
    LtEq(Position),                // <=
    GtEq(Position),                // >=
    AddEq(Position),               // +=
    SubEq(Position),               // -=
    MulEq(Position),               // *=
    DivEq(Position),               // /=
    ModEq(Position),               // %=
    RShiftEq(Position),            // >>=
    LShiftEq(Position),            // <<=
    RShift(Position),              // >>
    LShift(Position),              // <<
    NotEq(Position),               // !=
    OrEq(Position),                // |=
    AndEq(Position),               // &=
    XorEq(Position),               // ^=
    Add(Position),                 // +
    Sub(Position),                 // -
    Mul(Position),                 // *
    Div(Position),                 // /
    Mod(Position),                 // %
    AddAdd(Position),              // ++
    SubSub(Position),              // --
    Not(Position),                 // !
    Xor(Position),                 // ^
    Or(Position),                  // |
    OrOr(Position),                // ||
    And(Position),                 // &
    AndAnd(Position),              // &&
    Eof(Position),
}

macro_rules! next_and {
    ($iter:ident, $ret:ident) => {
        {
            $iter.next();
            $ret
        }
    };

    ($iter:ident, $ret:expr) => {
        {
            $iter.next();
            $ret
        }
    };
}

pub fn tokenize(input: String) -> Vec<Token> {
    let (mut row, mut col) = (1, 1);
    let mut output: Vec<Token> = vec![];
    let mut chars = input.chars().into_iter().peekable();
    while let Some(char) = chars.next() {
        match char {
            ' ' | '\n' | '\t' => {
                if char == '\n' {
                    row += 1;
                    col = 1;
                }
            },
            c if c.is_ascii_alphabetic() || c == '_' => {
                let start = (row, col);
                let mut val = String::from(char);
                while let Some(n) = chars.peek() {
                    if !n.is_ascii_alphanumeric() && *n != '_' { break; }
                    col += 1;
                    val.push(*n);
                    chars.next();
                }

                output.push(Token::Identifier(Position{ start, end: (row, col) }, val));
            },
            //TODO: decimals
            c if c.is_ascii_digit() => {
                let start = (row, col);
                let mut val = String::from(char);
                while let Some(n) = chars.peek() {
                    if !n.is_ascii_digit() { break; }
                    col += 1;
                    val.push(*n);
                    chars.next();
                }

                output.push(Token::NumLiteral(Position{ start, end: (row, col) }, val));
            },
            '"' => {
                let start = (row, col);
                let mut val = String::new();
                let mut terminated = false;
                while let Some(&n) = chars.peek() {
                    chars.next();
                    col += 1;
                    if n == '"' {
                        terminated = true;
                        break;
                    }

                    val.push(n);
                }

                if !terminated { panic!("ERROR: string literal not terminated at {row}:{col}"); }
                output.push(Token::StrLiteral(Position{ start, end: (row, col) }, val));
            },
            '(' => output.push(Token::LParen(Position{ start: (row, col), end: (row, col) })),
            ')' => output.push(Token::RParen(Position{ start: (row, col), end: (row, col) })),
            '{' => output.push(Token::LBrace(Position{ start: (row, col), end: (row, col) })),
            '}' => output.push(Token::RBrace(Position{ start: (row, col), end: (row, col) })),
            '<' => {
                let mut cur = Token::Lt(Position{ start: (row, col), end: (row, col) });
                if let Some(next) = chars.peek() {
                    cur = match next {
                        '=' => next_and!(chars, Token::LtEq(Position{ start: (row, col), end: (row + 1, col + 1) })),
                        '<' => {
                            chars.next();
                            if let Some('=') = chars.peek() {
                                next_and!(chars, Token::LShiftEq(Position{ start: (row, col), end: (row + 2, col + 2) }))
                            } else {
                                Token::LShift(Position{ start: (row, col), end: (row + 1, col + 1) })
                            }
                        }
                        _ => cur,
                    };
                }
                output.push(cur);
            },
            '>' => {
                let mut cur = Token::Gt(Position{ start: (row, col), end: (row, col) });
                if let Some(next) = chars.peek() {
                    cur = match next {
                        '=' => next_and!(chars, Token::GtEq(Position{ start: (row, col), end: (row + 1, col + 1) })),
                        '>' => {
                            chars.next();
                            if let Some('=') = chars.peek() {
                                next_and!(chars, Token::RShiftEq(Position{ start: (row, col), end: (row + 2, col + 2) })) }
                            else {
                                Token::RShift(Position{ start: (row, col), end: (row + 1, col + 1) })
                            }
                        }
                        _ => cur,
                    };
                }
                output.push(cur);
            },
            '+' => {
                let mut cur = Token::Add(Position{ start: (row, col), end: (row, col) });
                if let Some(next) = chars.peek() {
                    cur = match next {
                        '+' => next_and!(chars, Token::AddAdd(Position{ start: (row, col), end: (row + 1, col + 1) })),
                        '=' => next_and!(chars, Token::AddEq(Position{ start: (row, col), end: (row + 1, col + 1) })),
                        _ => cur,
                    };
                }
                output.push(cur);
            },
            '-' => {
                let mut cur = Token::Sub(Position{ start: (row, col), end: (row, col) });
                if let Some(next) = chars.peek() {
                    cur = match next {
                        '-' => next_and!(chars, Token::SubSub(Position{ start: (row, col), end: (row + 1, col + 1) })),
                        '=' => next_and!(chars, Token::SubEq(Position{ start: (row, col), end: (row + 1, col + 1) })),
                        '>' => next_and!(chars, Token::Arrow(Position{ start: (row, col), end: (row + 1, col + 1) })),
                        _ => cur,
                    };
                }
                output.push(cur);
            },
            '*' => {
                let mut cur = Token::Mul(Position{ start: (row, col), end: (row, col) });
                if let Some(next) = chars.peek() {
                    cur = match next {
                        '=' => next_and!(chars, Token::MulEq(Position{ start: (row, col), end: (row + 1, col + 1) })),
                        _ => cur,
                    }
                }
                output.push(cur);
            },
            '/' => {
                let mut cur = Token::Div(Position{ start: (row, col), end: (row, col) });
                if let Some(next) = chars.peek() {
                    cur = match next {
                        '/' => {
                            loop {
                                match chars.peek() {
                                    Some('\n') => break,
                                    None => break,
                                    _ => next_and!(chars, continue),
                                }
                            }

                            continue
                        },
                        '*' => {
                            loop {
                                match (chars.next(), chars.peek()) {
                                    (Some('*'), Some('/')) => break,
                                    (Some(_), Some(_)) => continue,
                                     _ => panic!("ERROR: comment block not terminated at {row}:{col}"),
                                }
                            }

                            continue
                        },
                        '=' => next_and!(chars, Token::DivEq(Position{ start: (row, col), end: (row + 1, col + 1) })),
                        _ => cur,
                    };
                }
                output.push(cur);
            },
            '=' => {
                let mut cur = Token::Eq(Position{ start: (row, col), end: (row, col) });
                if let Some(next) = chars.peek() {
                    cur = match next {
                        '=' => next_and!(chars, Token::EqEq(Position{ start: (row, col), end: (row + 1, col + 1) })),
                        '>' => next_and!(chars, Token::FatArrow(Position{ start: (row, col), end: (row + 1, col + 1) })),
                        _ => cur,
                    };
                } 
                output.push(cur);
            },
            '!' => {
                let mut cur = Token::Not(Position{ start: (row, col), end: (row, col) });
                if let Some(next) = chars.peek() {
                    cur = match next {
                        '=' => next_and!(chars, Token::NotEq(Position{ start: (row, col), end: (row + 1, col + 1) })),
                        _ => cur,
                    }
                }
                output.push(cur);
            },
            '|' => {
                let mut cur = Token::Or(Position{ start: (row, col), end: (row, col) });
                if let Some(next) = chars.peek() {
                    cur = match next {
                        '=' => next_and!(chars, Token::OrEq(Position{ start: (row, col), end: (row + 1, col + 1) })),
                        '|' => next_and!(chars, Token::OrOr(Position{ start: (row, col), end: (row + 1, col + 1) })),
                        _ => cur,
                    }
                }
                output.push(cur);
            },
            '&' => {
                let mut cur = Token::And(Position{ start: (row, col), end: (row, col) });
                if let Some(next) = chars.peek() {
                    cur = match next {
                        '=' => next_and!(chars, Token::AndEq(Position{ start: (row, col), end: (row + 1, col + 1) })),
                        '&' => next_and!(chars, Token::AndAnd(Position{ start: (row, col), end: (row + 1, col + 1) })),
                        _ => cur,
                    }
                } 
                output.push(cur);
            },
            '^' => {
                let mut cur = Token::Xor(Position{ start: (row, col), end: (row, col) });
                if let Some(next) = chars.peek() {
                    cur = match next {
                        '=' => next_and!(chars, Token::XorEq(Position{ start: (row, col), end: (row + 1, col + 1) })),
                        _ => cur,
                    }
                }
                output.push(cur);
            },
            '%' => {
                let mut cur = Token::Mod(Position{ start: (row, col), end: (row, col) });
                if let Some(next) = chars.peek() {
                    cur = match next {
                        '=' => next_and!(chars, Token::ModEq(Position{ start: (row, col), end: (row + 1, col + 1) })),
                        _ => cur,
                    };
                }
                output.push(cur);
            },
            _ => panic!("ERROR: unknown char `{char}` at {row}:{col}"),
        };

        col += 1;
    }

    output.push(Token::Eof(Position { start: (row, col), end: (row, col) }));
    output
}

#[cfg(test)]
mod tests {
    use std::mem::discriminant;
    use super::*;

    macro_rules! variant_eq {
        ($left:ident, $right:ident) => { discriminant(&$left) == discriminant(&$right) };
        ($left:ident, $right:expr) =>  { discriminant(&$left) == discriminant(&$right) };
        ($left:expr, $right:ident) =>  { discriminant(&$left) == discriminant(&$right) };
        ($left:expr, $right:expr) =>   { discriminant(&$left) == discriminant(&$right) };
    }


    #[test]
    fn operators() {
        let input = "-> => == <= >= += -= *= /= %= >>= <<= >> << != |= &= ^= ++ -- || &&".to_string();
        let tokens = tokenize(input);
        let mut token = tokens.iter();
        let pos = Position { start: (0, 0), end: (0, 0) };
        assert!(variant_eq!(*token.next().unwrap(), Token::Arrow(pos.clone())));
        assert!(variant_eq!(*token.next().unwrap(), Token::FatArrow(pos.clone())));
        assert!(variant_eq!(*token.next().unwrap(), Token::EqEq(pos.clone())));
        assert!(variant_eq!(*token.next().unwrap(), Token::LtEq(pos.clone())));
        assert!(variant_eq!(*token.next().unwrap(), Token::GtEq(pos.clone())));
        assert!(variant_eq!(*token.next().unwrap(), Token::AddEq(pos.clone())));
        assert!(variant_eq!(*token.next().unwrap(), Token::SubEq(pos.clone())));
        assert!(variant_eq!(*token.next().unwrap(), Token::MulEq(pos.clone())));
        assert!(variant_eq!(*token.next().unwrap(), Token::DivEq(pos.clone())));
        assert!(variant_eq!(*token.next().unwrap(), Token::ModEq(pos.clone())));
        assert!(variant_eq!(*token.next().unwrap(), Token::RShiftEq(pos.clone())));
        assert!(variant_eq!(*token.next().unwrap(), Token::LShiftEq(pos.clone())));
        assert!(variant_eq!(*token.next().unwrap(), Token::RShift(pos.clone())));
        assert!(variant_eq!(*token.next().unwrap(), Token::LShift(pos.clone())));
        assert!(variant_eq!(*token.next().unwrap(), Token::NotEq(pos.clone())));
        assert!(variant_eq!(*token.next().unwrap(), Token::OrEq(pos.clone())));
        assert!(variant_eq!(*token.next().unwrap(), Token::AndEq(pos.clone())));
        assert!(variant_eq!(*token.next().unwrap(), Token::XorEq(pos.clone())));
        assert!(variant_eq!(*token.next().unwrap(), Token::AddAdd(pos.clone())));
        assert!(variant_eq!(*token.next().unwrap(), Token::SubSub(pos.clone())));
        assert!(variant_eq!(*token.next().unwrap(), Token::OrOr(pos.clone())));
        assert!(variant_eq!(*token.next().unwrap(), Token::AndAnd(pos.clone())));
        assert!(variant_eq!(*token.next().unwrap(), Token::Eof(pos.clone())));
    }

    #[test]
    fn file() -> Result<(), std::io::Error> {
        let filename = "main.hl";
        let input = std::fs::read(filename)?;
        let input = std::str::from_utf8(input.as_slice()).expect("should be utf8").to_string();

        let tokens = tokenize(input);
        for token in tokens {
            eprintln!("{:?}", token);
        }

        Ok(())
    }
}
