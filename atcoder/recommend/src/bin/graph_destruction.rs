fn main() {
    let (n, m) = {
        let i = io::read::<usize>();
        (i[0], i[1])
    };

    let mut edges = vec![Vec::new(); n];
    for _ in 0..m {
        let (a, b) = {
            let i = io::read::<usize>();
            (i[0] - 1, i[1] - 1)
        };

        edges[a].push(b);
    }

    let mut uf = UnionFind::new(n);
    let mut ans = vec![0; n];
    let mut count = 0;

    for i in (1..n).rev() {
        count += 1;
        for &j in &edges[i] {
            if !uf.same(i, j) {
                uf.unite(i, j);
                count -= 1;
            }
        }

        ans[i - 1] = count;
    }

    for a in ans {
        println!("{}", a);
    }
}

pub struct UnionFind {
    par: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            par: (0..n).collect(),
            rank: vec![0; n],
        }
    }
    pub fn find(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            return x;
        }
        let y = self.par[x];
        self.par[x] = self.find(y);
        self.par[x]
    }
    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
    pub fn unite(&mut self, x: usize, y: usize) {
        use std::cmp::Ordering;

        let x = self.find(x);
        let y = self.find(y);
        if x == y {
            return;
        }
        match self.rank[x].cmp(&self.rank[y]) {
            Ordering::Less => {
                self.par[x] = y;
            }
            Ordering::Equal => {
                self.par[y] = x;
                self.rank[x] += 1;
            }
            Ordering::Greater => {
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
