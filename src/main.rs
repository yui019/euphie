use euphie::{env::*, eval::*, parse::*, tokenize::*};
use std::{cell::RefCell, fs, rc::Rc};

fn main() {
    let code = fs::read_to_string("test.lisp").expect("Could not read file.");
    let mut tokens = tokenize(code);

    tokens.reverse();
    let tree = parse(&mut tokens).unwrap();

    let result = eval_value(&tree, &mut Rc::from(RefCell::from(Env::new())));
    println!("Result: {:?}", result.unwrap());
}
