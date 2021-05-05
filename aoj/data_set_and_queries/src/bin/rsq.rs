use std::io;

fn main() {
    let (n, q) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let id = 0usize;
    let mut st = SegmentTree::new(n, id, |&x, &y| x + y);
    for _ in 0..q {
        let (com, x, y) = {
            let i = read::<usize>();
            (i[0], i[1], i[2])
        };
        if com == 0 {
            let v = st.get(x);
            st.update(x, y + v);
        } else {
            let m = st.apply(x, y + 1);
            println!("{}", m);
        }
    }
}

#[allow(dead_code)]
pub struct SegmentTree<T, F> {
    n: usize,
    v: Vec<T>,
    id: T,
    f: F,
}

impl<T, F> SegmentTree<T, F>
where
    T: Clone,
    F: Fn(&T, &T) -> T,
{
    pub fn new(n: usize, id: T, f: F) -> Self {
        let mut cap = 1;
        while cap <= n { cap *= 2; }
        SegmentTree {
            n: cap,
            v: vec![id.clone(); 2 * cap],
            id: id,
            f: f,
        }
    }

    #[inline]
    pub fn get(&self, i: usize) -> T {
        self.v[i + self.n].clone()
    }

    pub fn update(&mut self, mut i: usize, x: T) {
        i += self.n;
        self.v[i] = x;
        while { i /= 2; i > 0 } {
            self.v[i] = (self.f)(&self.v[i * 2], &self.v[i * 2 + 1]);
        }
    }

    pub fn apply(&mut self, s: usize, t: usize) -> T {
        let (mut x, mut y) = (self.id.clone(), self.id.clone());
        let (mut s, mut t) = (s + self.n, t + self.n);
        while s < t {
            if s % 2 != 0 {
                x = (self.f)(&x, &self.v[s]);
                s += 1;
            }
            if t % 2 != 0 {
                t -= 1;
                y = (self.f)(&self.v[t], &y);
            }
            s /= 2;
            t /= 2;
        }
        (self.f)(&x, &y)
    }
}

#[allow(dead_code)]
fn read<T>() -> Vec<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace()
        .map(|s| s.trim().parse().unwrap())
        .collect()
}

#[allow(dead_code)]
fn read_one<T>() -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().unwrap()
}
