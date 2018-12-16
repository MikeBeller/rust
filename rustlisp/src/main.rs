use std::rc::Rc;

#[derive(Debug,Clone)]
enum Value {
    Cons(Rc<(Value,Value)>),
    Number(i32),
    Symbol(Rc<String>),
    Nil,
}

use crate::Value::{Cons,Number,Nil,Symbol};

fn cons(car: Value, cdr: Value) -> Value {
    Cons(Rc::new((car,cdr)))
}

fn car(v: &Value) -> Value {
    match v {
        Cons(p) => p.0.clone(),
        _ => panic!("not a cons cell"),
    }
}

fn cdr(v: &Value) -> Value {
    match v {
        Cons(p) => p.1.clone(),
        _ => panic!("not a cons cell"),
    }
}

fn sym(s: &str) -> Value {
    Symbol(Rc::new(s.to_string()))
}

fn main() {
    let p1 = cons(sym("foo"), Number(3));
    let p2 = cons(sym("bar"), Number(7));
    let lst = cons(p1, cons(p2, Nil));
    println!("lst: {:?}", lst);
    println!("car lst: {:?}", car(&lst));
    println!("cdr lst: {:?}", cdr(&lst));
}
