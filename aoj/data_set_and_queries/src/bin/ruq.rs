use std::io;


fn main() {
}

/// http://tsutaj.hatenablog.com/entry/2017/03/30/224339
pub struct LazySegmentTree<T, F> {
    n: usize,
    node: Vec<T>,
    lazy: Vec<T>,
    id: T,
    f: F,
}

impl<T, F> LazySegmentTree<T, F>
where
    T: Clone + std::ops::AddAssign,
    F: Fn(&T, &T) -> T,
{

    pub fn new(n: usize, id: T, f: F) -> Self {
        let mut cap = 1;
        while cap <= n { cap *= 2; }
        LazySegmentTree {
            n: cap,
            node: vec![id.clone(); 2 * cap],
            lazy: vec![id.clone(); 2* cap],
            id: id,
            f: f,
        }
    }

    pub fn from(v: &[T], id: T, f: F) -> Self {
        let n = v.len();
        let mut vs = vec![id.clone(); n];
        vs.extend_from_slice(v);
        for i in (1..n).rev() {
            vs[i] = f(&vs[i * 2], &vs[i * 2 + 1]);
        }
        LazySegmentTree {
            n: n,
            node: vs,
            lazy: vec![id.clone(); 2 * n],
            id: id,
            f: f,
        }
    }

    fn eval(&mut self, k: usize, s: usize, t: usize) {
        if self.lazy[k] != self.id {
            self.node[k] += self.lazy[k];
        }
    }
}


#[allow(dead_code)]
fn read<T>() -> Vec<T>
where T:
std::str::FromStr,
T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace()
        .map(|s| s.trim().parse().unwrap())
        .collect()
}

#[allow(dead_code)]
fn read_one<T>() -> T
where T:
std::str::FromStr,
T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().unwrap()
}
