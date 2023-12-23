use crate::{env::*, eval::function::*, eval::misc::*, eval::op::*, eval::quote::*, parse::*};
use std::{cell::RefCell, rc::Rc};

mod function;
mod misc;
mod op;
mod quote;

fn eval_list(list: &[Value], env: &mut Rc<RefCell<Env>>) -> Result<Value, String> {
    // handle empty list
    if list.len() == 0 {
        return Ok(Value::Nil);
    }

    let head = &list[0];
    match head {
        Value::Symbol(s) => match s.as_str() {
            "+" | "-" | "*" | "/" => {
                return eval_arithmetic_op(&list, env);
            }
            "=" | "!=" | "<" | ">" | "<=" | ">=" => {
                return eval_comparison_op(&list, env);
            }
            "and" | "or" | "not" => {
                todo!()
            }
            "if" => eval_if(&list, env),
            "def" => eval_def(&list, env),
            "lambda" => eval_fun_definition(&list, env),
            "macro" => eval_macro_definition(&list, env),
            "macroexpand" => eval_macro_expand(&list, env),
            "let" => eval_let(&list, env),
            "quote" => eval_quote(&list, env),
            "quasiquote" => eval_quasiquote(&list, env),
            _ => eval_fun_call(&list, env),
        },

        _ => eval_fun_call(&list, env),
    }
}

pub fn eval_value(value: &Value, env: &mut Rc<RefCell<Env>>) -> Result<Value, String> {
    match value {
        Value::Nil => Ok(Value::Nil),
        Value::T => Ok(Value::T),
        Value::Number(n) => Ok(Value::Number(*n)),
        Value::String(s) => Ok(Value::String(s.clone())),
        Value::Symbol(s) => eval_symbol(s, env),
        Value::Lambda {
            params,
            body,
            is_macro,
        } => Ok(Value::Lambda {
            params: params.clone(),
            body: body.clone(),
            is_macro: is_macro.clone(),
        }),
        Value::List(l) => eval_list(l, env),
    }
}
