
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
    Nil,
}

use crate::Value::{Cons,Number,Nil};

fn cons(car: Value, cdr: Value) -> Value {
    Cons(Rc::new(Pair{car: car, cdr: cdr}))
}

fn car(v: &Value) -> Value {
    match v {
        Cons(p) => p.car.clone(),
        _ => Nil
    }
}

fn main() {
    let list = Nil;
    let num = Number(7);
    let c = cons(Nil,Nil);
    println!("{:?} {:?} {:?} {:?}", list, num, c, car(&c));
}
