use std::str;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    String(String),
    Bool(bool),
    Symbol(String),
    StartParen,
    EndParen,
}

pub fn tokenize(code: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];

    let mut i = 0;
    while i < code.len() {
        let c = code.as_bytes()[i] as char;

        if c.is_whitespace() {
            i += 1;
            continue;
        }

        if c == '(' {
            tokens.push(Token::StartParen);
        } else if c == ')' {
            tokens.push(Token::EndParen);
        } else if c == '"' {
            let mut j = i + 1;
            while j < code.len() {
                if code.as_bytes()[j] == b'"' {
                    break;
                }

                j += 1;
            }

            let substr = String::from(str::from_utf8(&code.as_bytes()[(i + 1)..j]).unwrap());
            tokens.push(Token::String(substr));
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

            if substr == "true" {
                tokens.push(Token::Bool(true))
            } else if substr == "false" {
                tokens.push(Token::Bool(false))
            } else {
                match substr.parse::<f64>() {
                    Ok(n) => tokens.push(Token::Number(n)),
                    Err(_) => tokens.push(Token::Symbol(substr)),
                }
            }

            i = j - 1;
        }

        i += 1;
    }

    tokens
}
