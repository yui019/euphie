use crate::parse::Value;

pub fn value_to_string(value: &Value) -> String {
    match value {
        Value::Nil => format!("nil"),
        Value::T => format!("t"),
        Value::Number(n) => format!("{}", n),
        Value::String(s) => format!("\"{}\"", s),
        Value::Symbol(s) => format!("{}", s),
        Value::Lambda {
            params, is_macro, ..
        } => format!(
            "<{} ({})>",
            if *is_macro { "macro" } else { "lambda" },
            params.join(" ")
        ),
        Value::List(l) => format!(
            "({})",
            l.iter()
                .map(value_to_string)
                .collect::<Vec<String>>()
                .join(" ")
        ),
    }
}
