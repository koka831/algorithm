use std::io;
use std::cmp;


fn main() {
    let (n, m) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    let mut ab = Vec::new();

    for _ in 0..m {
        let (a, b) = {
            let i = read::<usize>();
            (i[0], i[1])
        };
        ab.push((a - 1, b - 1));
    }

    ab.reverse();

    let mut cost = Vec::new();

    let mut uf = UnionFind::new(n);
    for &(a, b) in ab.iter() {
        let before_a = uf.find(a);
        let before_b = uf.find(b);
        let c = uf.siz[before_a] * uf.siz[before_b];
        uf.unite(a, b);
        let after_a = uf.find(a);
        let after_b = uf.find(b);

        let c = if before_a != after_a || before_b != after_b {
            c
        } else {
            0
        };
        cost.push(c);
    }
    cost.reverse();

    let mut ans: usize = 0;
    for i in 0..cost.len() {
        ans += cost[i];
        println!("{}", ans);
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

#[derive(Debug)]
pub struct UnionFind {
    pub par: Vec<usize>,
    pub rank: Vec<usize>,
    pub siz: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        UnionFind {
            par: (0..n).collect(),
            rank: vec![0; n],
            siz: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.par[x] == x { x }
        else {
            let p = self.par[x];
            let pp = self.find(p);
            self.par[x] = pp;
            pp
        }
    }

    fn unite(&mut self, x: usize, y: usize) {
        let x = self.find(x);
        let y = self.find(y);

        if x == y { return; }

        let prev_x = self.siz[x];
        let prev_y = self.siz[y];
        self.siz[x] += prev_y;
        self.siz[y] += prev_x;

        match self.rank[x].cmp(&self.rank[y]) {
            cmp::Ordering::Equal => {
                self.par[y] = x;
                self.rank[x] += 1;
            },
            cmp::Ordering::Less => {
                self.par[x] = y;
            },
            cmp::Ordering::Greater => {
                self.par[y] = x;
            }
        }

    }
}
