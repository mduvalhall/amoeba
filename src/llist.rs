#[derive(Debug)]
pub enum List<T> {
    Nil,
    Cons(T, Box<List<T>>),
}

use List::*;

pub fn rev<T>(list: List<T>, acc: List<T>) -> List<T> {
    match list {
        Nil => Nil,
        Cons(hd, tl) => rev(*tl, Cons(hd, Box::new(acc))),
    }
}

fn main() {
    let l = Cons(1, Box::new(Cons(2, Box::new(Nil))));
    let l = rev(l, Nil);
    dbg!(l);
}
