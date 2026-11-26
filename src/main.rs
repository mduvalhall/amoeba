#[derive(Debug)]
pub enum List<T> {
    Nil,
    Cons(T, Box<List<T>>),
}

use std::fmt::Debug;

use List::*;

pub fn cons<T>(value: T, list: List<T>) -> List<T> {
    Cons(value, Box::new(list))
}

fn rev<T>(list: List<T>) -> List<T> {
    fn aux<T>(list: List<T>, acc: List<T>) -> List<T> {
        match list {
            Nil => acc,
            Cons(hd, tl) => aux(*tl, Cons(hd, Box::new(acc))),
        }
    }
    aux(list, Nil)
}

fn iter<T>(list: &List<T>, f: impl Fn(&T)) {
    match list {
        Nil => (),
        Cons(hd, tl) => {
            f(hd);
            iter(tl, f);
        }
    }
}

fn map<T>(list: List<T>, f: impl Fn(T) -> T) -> List<T> {
    fn aux<T>(list: List<T>, acc: List<T>, f: impl Fn(T) -> T) -> List<T> {
        match list {
            Nil => rev(acc),
            Cons(hd, tl) => {
                let hd = f(hd);
                aux(*tl, cons(hd, acc), f)
            }
        }
    }
    aux(list, Nil, f)
}

impl<const N: usize, T> From<[T; N]> for List<T> {
    fn from(value: [T; N]) -> Self {
        let mut l = Nil;
        for v in value.into_iter().rev() {
            l = cons(v, l)
        }
        l
    }
}

fn main() {
    let l = List::from([1, 2, 3, 4, 5]);
    let l = rev(l);
    iter(&l, |v| print!("{v} "));
    println!();
    let l = map(l, |v| v * 2);
    iter(&l, |v| print!("{v} "));
    println!();
}
