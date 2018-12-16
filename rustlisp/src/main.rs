
use std::rc::Rc;

#[derive(Debug)]
struct Pair {
    car: Value,
    cdr: Value,
}

#[derive(Debug,Clone)]
enum Value {
    Cons(Rc<Pair>),
    Number(i32),
    Symbol(Rc<String>),
    Nil,
}

use crate::Value::{Cons,Number,Nil,Symbol};

fn cons(car: Value, cdr: Value) -> Value {
    Cons(Rc::new(Pair{car: car, cdr: cdr}))
}

fn car(v: &Value) -> Value {
    match v {
        Cons(p) => p.car.clone(),
        _ => panic!("not a list"),
    }
}

fn cdr(v: &Value) -> Value {
    match v {
        Cons(p) => p.cdr.clone(),
        _ => panic!("not a list"),
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
