use std::io;

fn main() {
    let (_n, q) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    let cn = read::<usize>();

    for _ in 0..q {
        let (l, r) = {
            let i = read::<usize>();
            (i[0], i[1])
        };
    }
}

#[derive(Debug)]
struct FenwickTree {
    n: usize,
    bit: Vec<usize>,
}

impl FenwickTree {
    fn new(n: usize) -> Self {
        FenwickTree { n, bit: vec![0; n] }
    }

    // [0, k)
    fn sum(&self, k: usize) -> usize {
        let mut s = 0;
        let mut i = k;
        while i > 0 {
            i -= 1;
            s += self.bit[i];
            i &= i + 1;
        }
        s
    }

    fn add(&mut self, i: usize, v: usize) {
        let mut i = i;
        while i < self.n {
            self.bit[i] += v;
            i |= i + 1;
        }
    }
}


#[allow(unused)]
fn read<T>() -> Vec<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug {

        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        buf.split_whitespace()
            .map(|s| s.trim().parse().unwrap())
            .collect()
}

#[allow(unused)]
fn read_one<T>() -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug {

        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        buf.trim().parse().unwrap()
}
