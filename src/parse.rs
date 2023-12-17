use crate::tokenize::Token;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Nil,
    T,
    Number(f64),
    String(String),
    Symbol(String),
    Lambda {
        params: Vec<String>,
        body: Rc<Value>,
        is_macro: bool,
    },
    List(Vec<Value>),
}

pub fn parse(tokens: &mut Vec<Token>) -> Result<Value, String> {
    if tokens.len() > 1 && tokens[tokens.len() - 1] != Token::StartParen {
        return Err(String::from("Expected '(' at beginning of file"));
    }

    match tokens.pop().unwrap() {
        Token::T => Ok(Value::T),
        Token::Nil => Ok(Value::Nil),
        Token::Number(n) => Ok(Value::Number(n)),
        Token::String(s) => Ok(Value::String(s)),
        Token::Symbol(s) => Ok(Value::Symbol(s)),

        Token::StartParen => {
            let mut list: Vec<Value> = vec![];

            loop {
                let token = tokens.pop();

                match token {
                    None => return Err(String::from("Expected ')' at end of file")),

                    Some(Token::Nil) => list.push(Value::Nil),
                    Some(Token::T) => list.push(Value::T),
                    Some(Token::Number(n)) => list.push(Value::Number(n)),
                    Some(Token::String(s)) => list.push(Value::String(s)),
                    Some(Token::Symbol(s)) => list.push(Value::Symbol(s)),

                    Some(Token::StartParen) => {
                        tokens.push(Token::StartParen);
                        list.push(parse(tokens)?);
                    }

                    Some(Token::EndParen) => break,
                }
            }

            Ok(Value::List(list))
        }

        Token::EndParen => Err(String::from("Unexpected ')'")),
    }
}
