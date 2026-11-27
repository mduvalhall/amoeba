enum List<A> {
    Empty,
    Node(A, Box<List<A>>),
}

fn fold<A, Acc>(list: &List<A>, acc: Acc, f: impl Fn(Acc, &A) -> Acc) -> Acc {
    let (mut list, mut acc) = (list, acc);
    while let List::Node(hd, tl) = list {
        list = tl;
        acc = f(acc, hd);
    }
    acc
}

fn fold2<A, B, Acc>(l1: &List<A>, l2: &List<B>, acc: Acc, f: impl Fn(Acc, &A, &B) -> Acc) -> Acc {
    let (mut l1, mut l2, mut acc) = (l1, l2, acc);
    while let (List::Node(hd1, tl1), List::Node(hd2, tl2)) = (l1, l2) {
        (l1, l2) = (tl1, tl2);
        acc = f(acc, hd1, hd2);
    }
    acc
}

fn iter<A>(list: &List<A>, f: impl Fn(&A)) {
    fold(list, (), |(), v| f(v))
}

fn rev<A: Clone>(list: &List<A>) -> List<A> {
    fold(list, List::Empty, |l, v| List::Node(v.clone(), Box::new(l)))
}

fn rev_owned<A>(list: List<A>) -> List<A> {
    let (mut list, mut out) = (list, List::Empty);
    while let List::Node(hd, mut tl) = std::mem::replace(&mut list, List::Empty) {
        list = std::mem::replace(&mut *tl, List::Empty);
        out = List::Node(hd, Box::new(out))
    }
    out
}

fn map<A, B>(list: &List<A>, f: impl Fn(&A) -> B) -> List<B> {
    rev_owned(fold(&list, List::Empty, |l, v| {
        List::Node(f(v), Box::new(l))
    }))
}

fn zip<A: Clone, B: Clone>(l1: &List<A>, l2: &List<B>) -> List<(A, B)> {
    rev_owned(fold2(&l1, &l2, List::Empty, |l, a, b| {
        List::Node((a.clone(), b.clone()), Box::new(l))
    }))
}

macro_rules! list {
    () => { List::Empty };
    ($head:expr $(, $tail:expr)* $(,)?) => {
        List::Node($head, Box::new(list!($($tail),*)))
    };
}

fn main() {
    let l1 = list![1, 2, 3, 4, 5];
    iter(&l1, |v| print!("{v} "));
    println!();

    let l2 = rev(&l1);
    iter(&l2, |v| print!("{v} "));
    println!();

    let sum = fold(&l1, 0, |acc, v| acc + v);
    println!("{sum}");

    let l3 = map(&l1, |v| v + 10);
    iter(&l3, |v| print!("{v} "));
    println!();

    let l4 = zip(&l1, &l2);
    iter(&l4, |(a, b)| print!("({a}, {b}) "));
    println!();

    let l5 = list!["hello", "world"];
    let l6 = list!["marco", "polo"];
    let l7 = zip(&l5, &l6);
    iter(&l7, |(a, b)| print!("({a}, {b}) "));
    println!();
}
