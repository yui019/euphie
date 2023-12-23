use crate::{tokenize::*, util::value_to_string};
use core::fmt::Debug;
use std::rc::Rc;

#[derive(Clone, PartialEq)]
pub struct LambdaParams {
    pub required: Vec<String>,
    pub optional: Vec<String>,
    pub keyword: Vec<String>,
    pub rest: Option<String>,
}

impl Debug for LambdaParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "required: [{}], optional: [{}], keyword: [{}], rest: [{}]",
            self.required.join(" "),
            self.optional.join(" "),
            self.keyword.join(" "),
            self.rest.clone().unwrap_or(String::from(""))
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Nil,
    T,
    Number(f64),
    String(String),
    Symbol(String),
    Lambda {
        params: LambdaParams,
        body: Rc<Value>,
        is_macro: bool,
    },
    List(Vec<Value>),
}

fn wrap_value_with_prefix(value: &Value, prefix: &[char]) -> Value {
    if prefix.len() == 0 {
        return value.clone();
    }

    let quote = '\'';
    let backtick = '`';
    let comma = ',';
    let at = '@';

    if prefix[0] == quote {
        if prefix.len() > 1 {
            // if there's more things after the ', then treat the entire expression, along with rest of the prefix, as a single symbol
            let mut string = String::with_capacity(prefix.len() - 1);
            for c in &prefix[1..] {
                string.insert(string.len(), *c);
            }
            string += &value_to_string(&value);
            return Value::Symbol(string);
        } else {
            // otherwise, just quote the expression
            return Value::List(vec![Value::Symbol(format!("quote")), value.clone()]);
        }
    }

    if prefix[0] == backtick {
        if prefix.len() > 1 {
            // if there's more things after the `, wrap this same function recursively (without the 1st prefix character) in a quasiquote
            return Value::List(vec![
                Value::Symbol(format!("quasiquote")),
                wrap_value_with_prefix(&value, &prefix[1..]),
            ]);
        } else {
            // otherwise, just wrap this value in a quasiquote
            return Value::List(vec![Value::Symbol(format!("quasiquote")), value.clone()]);
        }
    }

    if prefix[0] == comma {
        if prefix.len() > 1 {
            if prefix[1] == at {
                if prefix.len() > 2 {
                    return Value::List(vec![
                        Value::Symbol(format!("splice-unquote")),
                        wrap_value_with_prefix(&value, &prefix[2..]),
                    ]);
                } else {
                    return Value::List(vec![
                        Value::Symbol(format!("splice-unquote")),
                        value.clone(),
                    ]);
                }
            } else {
                return Value::List(vec![
                    Value::Symbol(format!("unquote")),
                    wrap_value_with_prefix(&value, &prefix[1..]),
                ]);
            }
        } else {
            return Value::List(vec![Value::Symbol(format!("unquote")), value.clone()]);
        }
    }

    Value::Nil
}

pub fn parse(tokens: &mut Vec<Token>) -> Result<Value, String> {
    if tokens.len() > 1 && tokens[tokens.len() - 1].t != TokenType::StartParen {
        return Err(String::from("Expected '(' at beginning of file"));
    }

    let token = tokens.pop().unwrap();
    match token.t {
        TokenType::T => Ok(wrap_value_with_prefix(&Value::T, &token.prefix)),
        TokenType::Nil => Ok(wrap_value_with_prefix(&Value::Nil, &token.prefix)),
        TokenType::Number(n) => Ok(wrap_value_with_prefix(&Value::Number(n), &token.prefix)),
        TokenType::String(s) => Ok(wrap_value_with_prefix(&Value::String(s), &token.prefix)),
        TokenType::Symbol(s) => Ok(wrap_value_with_prefix(&Value::Symbol(s), &token.prefix)),

        TokenType::StartParen => {
            let mut list: Vec<Value> = vec![];

            loop {
                let token = tokens.pop();

                match token {
                    None => return Err(String::from("Expected ')' at end of file")),

                    Some(tkn) => match tkn.t {
                        TokenType::Nil => {
                            list.push(wrap_value_with_prefix(&Value::Nil, &tkn.prefix))
                        }
                        TokenType::T => list.push(wrap_value_with_prefix(&Value::T, &tkn.prefix)),
                        TokenType::Number(n) => {
                            list.push(wrap_value_with_prefix(&Value::Number(n), &tkn.prefix))
                        }
                        TokenType::String(s) => {
                            list.push(wrap_value_with_prefix(&Value::String(s), &tkn.prefix))
                        }
                        TokenType::Symbol(s) => {
                            list.push(wrap_value_with_prefix(&Value::Symbol(s), &tkn.prefix))
                        }

                        TokenType::StartParen => {
                            tokens.push(tkn.clone());
                            list.push(parse(tokens)?);
                        }

                        TokenType::EndParen => break,
                    },
                }
            }

            Ok(wrap_value_with_prefix(&Value::List(list), &token.prefix))
        }

        TokenType::EndParen => Err(String::from("Unexpected ')'")),
    }
}
