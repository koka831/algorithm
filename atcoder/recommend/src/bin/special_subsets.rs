const modulo: usize = 998_244_353;

fn main() {
    let n = io::read_one::<usize>();
    let fs = io::read::<usize>();

    let mut uf = UnionFind::new(n);

    for i in 0..n {
        uf.unite(i, fs[i] - 1);
    }

    let mut ans = 1;
    let mut visit = vec![false; n];
    for i in 0..n {
        let p = uf.find(i);
        if visit[p] { continue; }

        visit[p] = true;
        ans *= 2;
        ans %= modulo;
    }

    println!("{}", ans - 1);
}

#[derive(Debug)]
struct UnionFind {
    par: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            par: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.par[x] == x { x }
        else {
        let y = self.par[x];
        self.par[x] = self.find(y);
        self.par[x]
        }
    }

    fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    fn unite(&mut self, x: usize, y: usize) {
        use std::cmp;
        let x = self.find(x);
        let y = self.find(y);
        if x == y { return; }

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

#[allow(unused)]
mod io {
    use std::io;
    pub fn read<T>() -> Vec<T>
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
    pub fn read_one<T>() -> T
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        buf.trim().parse().unwrap()
    }
}
