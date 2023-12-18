use std::str;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Nil,
    T,
    Number(f64),
    String(String),
    Symbol(String),
    StartParen,
    EndParen,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub t: TokenType,
    pub prefix: Vec<char>,
    // TODO: line and column number
}

pub fn tokenize(code: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];

    // these are all used as single char prefixes, with the exception of splice-unquote, which is ,@
    const PREFIX_CHARS: [char; 4] = ['\'', '`', ',', '@'];

    let mut i = 0;
    let mut prefix: Vec<char> = vec![];
    while i < code.len() {
        let c = code.as_bytes()[i] as char;

        if c.is_whitespace() {
            i += 1;
            prefix.clear();
            continue;
        }

        if PREFIX_CHARS.contains(&c) {
            i += 1;
            prefix.push(c);
            continue;
        }

        if c == '(' {
            tokens.push(Token {
                t: TokenType::StartParen,
                prefix: prefix.clone(),
            });
            prefix.clear();
        } else if c == ')' {
            tokens.push(Token {
                t: TokenType::EndParen,
                prefix: prefix.clone(),
            });
            prefix.clear();
        } else if c == '"' {
            let mut j = i + 1;
            while j < code.len() {
                if code.as_bytes()[j] == b'"' {
                    break;
                }

                j += 1;
            }

            let substr = String::from(str::from_utf8(&code.as_bytes()[(i + 1)..j]).unwrap());
            tokens.push(Token {
                t: TokenType::String(substr),
                prefix: prefix.clone(),
            });
            prefix.clear();
            i = j;
        } else {
            let mut j = i + 1;
            while j < code.len() {
                let inner_c = code.as_bytes()[j] as char;
                if inner_c.is_whitespace() || inner_c == ')' {
                    break;
                }

                j += 1;
            }

            let substr = String::from(str::from_utf8(&code.as_bytes()[i..j]).unwrap());

            if substr == "nil" {
                tokens.push(Token {
                    t: TokenType::Nil,
                    prefix: prefix.clone(),
                });
                prefix.clear();
            } else if substr == "t" {
                tokens.push(Token {
                    t: TokenType::T,
                    prefix: prefix.clone(),
                });
                prefix.clear();
            } else {
                match substr.parse::<f64>() {
                    Ok(n) => {
                        tokens.push(Token {
                            t: TokenType::Number(n),
                            prefix: prefix.clone(),
                        });

                        prefix.clear();
                    }
                    Err(_) => {
                        tokens.push(Token {
                            t: TokenType::Symbol(substr),
                            prefix: prefix.clone(),
                        });
                        prefix.clear();
                    }
                }
            }

            i = j - 1;
        }

        i += 1;
    }

    tokens
}
