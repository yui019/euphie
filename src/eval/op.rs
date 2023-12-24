use super::eval_value;
use crate::{env::*, parse::*};
use std::{cell::RefCell, rc::Rc};

pub fn eval_arithmetic_op(list: &[Value], env: &mut Rc<RefCell<Env>>) -> Result<Value, String> {
    let head = &list[0];
    let tail = &list[1..];

    match head {
        Value::Symbol(s) => match s.as_str() {
            "+" => {
                let mut r: f64 = 0.0;
                for v in tail {
                    match eval_value(v, env)? {
                        Value::Number(n) => r += n,
                        _ => return Err(String::from("All arguments must be numbers")),
                    }
                }

                Ok(Value::Number(r))
            }

            "-" => {
                // return the negative if there's only 1 argument
                if tail.len() == 1 {
                    return match eval_value(&tail[0], env)? {
                        Value::Number(n) => Ok(Value::Number(-n)),
                        _ => return Err(String::from("All arguments must be numbers")),
                    };
                }

                let mut r: f64 = match eval_value(&tail[0], env)? {
                    Value::Number(n) => n,
                    _ => return Err(String::from("All arguments must be numbers")),
                };

                for v in &tail[1..] {
                    match eval_value(v, env)? {
                        Value::Number(n) => r -= n,
                        _ => return Err(String::from("All arguments must be numbers")),
                    }
                }

                Ok(Value::Number(r))
            }

            "*" => {
                let mut r: f64 = 1.0;
                for v in tail {
                    match eval_value(v, env)? {
                        Value::Number(n) => r *= n,
                        _ => return Err(String::from("All arguments must be numbers")),
                    }
                }

                Ok(Value::Number(r))
            }

            "/" => {
                let mut r: f64 = match eval_value(&tail[0], env)? {
                    Value::Number(n) => n,
                    _ => return Err(String::from("All arguments must be numbers")),
                };
                for v in &tail[1..] {
                    match eval_value(v, env)? {
                        Value::Number(n) => r /= n,
                        _ => return Err(String::from("All arguments must be numbers")),
                    }
                }

                Ok(Value::Number(r))
            }

            _ => unreachable!(),
        },

        _ => unreachable!(),
    }
}

pub fn eval_comparison_op(list: &[Value], env: &mut Rc<RefCell<Env>>) -> Result<Value, String> {
    let head = &list[0];
    let tail = &list[1..];

    match head {
        Value::Symbol(s) => match s.as_str() {
            ">" => {
                let mut r: bool = true;
                if tail.len() < 2 {
                    return Err(String::from("\">\" requires at least 2 arguments"));
                }

                for i in 1..tail.len() {
                    match (eval_value(&tail[i - 1], env)?, eval_value(&tail[i], env)?) {
                        (Value::Number(a), Value::Number(b)) => {
                            if !(a > b) {
                                r = false;
                                break;
                            }
                        }
                        _ => return Err(String::from("All arguments must be numbers")),
                    }
                }

                if r {
                    Ok(Value::T)
                } else {
                    Ok(Value::Nil)
                }
            }

            "<" => {
                let mut r: bool = true;
                if tail.len() < 2 {
                    return Err(String::from("\"<\" requires at least 2 arguments"));
                }

                for i in 1..tail.len() {
                    match (eval_value(&tail[i - 1], env)?, eval_value(&tail[i], env)?) {
                        (Value::Number(a), Value::Number(b)) => {
                            if !(a < b) {
                                r = false;
                                break;
                            }
                        }
                        _ => return Err(String::from("All arguments must be numbers")),
                    }
                }

                if r {
                    Ok(Value::T)
                } else {
                    Ok(Value::Nil)
                }
            }

            ">=" => {
                let mut r: bool = true;
                if tail.len() < 2 {
                    return Err(String::from("\">=\" requires at least 2 arguments"));
                }

                for i in 1..tail.len() {
                    match (eval_value(&tail[i - 1], env)?, eval_value(&tail[i], env)?) {
                        (Value::Number(a), Value::Number(b)) => {
                            if !(a >= b) {
                                r = false;
                                break;
                            }
                        }
                        _ => return Err(String::from("All arguments must be numbers")),
                    }
                }

                if r {
                    Ok(Value::T)
                } else {
                    Ok(Value::Nil)
                }
            }

            "<=" => {
                let mut r: bool = true;
                if tail.len() < 2 {
                    return Err(String::from("\"<=\" requires at least 2 arguments"));
                }

                for i in 1..tail.len() {
                    match (eval_value(&tail[i - 1], env)?, eval_value(&tail[i], env)?) {
                        (Value::Number(a), Value::Number(b)) => {
                            if !(a <= b) {
                                r = false;
                                break;
                            }
                        }
                        _ => return Err(String::from("All arguments must be numbers")),
                    }
                }

                if r {
                    Ok(Value::T)
                } else {
                    Ok(Value::Nil)
                }
            }

            "=" => {
                let mut r: bool = true;
                if tail.len() < 2 {
                    return Err(String::from("\"=\" requires at least 2 arguments"));
                }

                for i in 1..tail.len() {
                    match (eval_value(&tail[i - 1], env)?, eval_value(&tail[i], env)?) {
                        (Value::Number(a), Value::Number(b)) => {
                            if !(a == b) {
                                r = false;
                                break;
                            }
                        }
                        _ => return Err(String::from("All arguments must be numbers")),
                    }
                }

                if r {
                    Ok(Value::T)
                } else {
                    Ok(Value::Nil)
                }
            }

            "!=" => {
                let mut r: bool = true;
                if tail.len() < 2 {
                    return Err(String::from("\"!=\" requires at least 2 arguments"));
                }

                for i in 1..tail.len() {
                    match (eval_value(&tail[i - 1], env)?, eval_value(&tail[i], env)?) {
                        (Value::Number(a), Value::Number(b)) => {
                            if !(a != b) {
                                r = false;
                                break;
                            }
                        }
                        _ => return Err(String::from("All arguments must be numbers")),
                    }
                }

                if r {
                    Ok(Value::T)
                } else {
                    Ok(Value::Nil)
                }
            }

            _ => unreachable!(),
        },

        _ => unreachable!(),
    }
}

pub fn eval_logic_op(list: &[Value], env: &mut Rc<RefCell<Env>>) -> Result<Value, String> {
    let head = &list[0];
    let tail = &list[1..];

    match head {
        Value::Symbol(s) => match s.as_str() {
            "and" => {
                for v in tail {
                    match eval_value(v, env)? {
                        // early return if it's nil
                        Value::Nil => return Ok(Value::Nil),
                        _ => {}
                    }
                }

                // return the value of the last element
                Ok(tail[tail.len() - 1].clone())
            }

            "or" => {
                for v in tail {
                    match eval_value(v, env)? {
                        // skip if it's nil
                        Value::Nil => {}
                        // early return if it's not nil
                        value => return Ok(value),
                    }
                }

                Ok(Value::Nil)
            }

            "not" => {
                if tail.len() != 1 {
                    return Err(String::from("\"not\" requires 1 argument"));
                }

                match tail[0] {
                    Value::Nil => Ok(Value::T),
                    _ => Ok(Value::Nil),
                }
            }

            _ => unreachable!(),
        },

        _ => unreachable!(),
    }
}

pub fn eval_list_op(list: &[Value], env: &mut Rc<RefCell<Env>>) -> Result<Value, String> {
    let head = &list[0];
    let tail = &list[1..];

    match head {
        Value::Symbol(s) => match s.as_str() {
            "car" => {
                if tail.len() != 1 {
                    return Err(String::from("\"car\" requires 1 argument"));
                }

                match eval_value(&tail[0], env)? {
                    Value::List(l) => {
                        if l.len() == 0 {
                            Ok(Value::Nil)
                        } else {
                            Ok(l[0].clone())
                        }
                    }
                    _ => Err(String::from("Argument needs to be a list")),
                }
            }

            "cdr" => {
                if tail.len() != 1 {
                    return Err(String::from("\"cdr\" requires 1 argument"));
                }

                match eval_value(&tail[0], env)? {
                    Value::List(l) => {
                        if l.len() < 2 {
                            Ok(Value::Nil)
                        } else {
                            Ok(Value::List(l[1..].to_vec()))
                        }
                    }
                    _ => Err(String::from("Argument needs to be a list")),
                }
            }

            "len" => {
                if tail.len() != 1 {
                    return Err(String::from("\"len\" requires 1 argument"));
                }

                match eval_value(&tail[0], env)? {
                    Value::List(l) => Ok(Value::Number(l.len() as f64)),
                    _ => Err(String::from("Argument needs to be a list")),
                }
            }

            _ => unreachable!(),
        },

        _ => unreachable!(),
    }
}
