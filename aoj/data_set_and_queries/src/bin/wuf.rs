use std::io;


fn main() {
    let (n, q) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let mut uf = WeightedUnionFind::new(n);

    for _ in 0..q {
        let q = read::<usize>();
        if q[0] == 0 {
            uf.unite(q[1], q[2], q[3] as isize);
        } else {
            let (x, y) = (q[1], q[2]);
            if uf.same(x, y) {
                println!("{}", uf.weight(y) - uf.weight(x));
            } else {
                println!("?");
            }
        }
    }
}

struct WeightedUnionFind {
    par: Vec<usize>,
    rank: Vec<usize>,
    w: Vec<isize>,
}

impl WeightedUnionFind {
    fn new(n: usize) -> Self {
        WeightedUnionFind {
            par: (0..n).collect(),
            rank: vec![0; n],
            w: vec![0; n],
        }
    }

    fn unite(&mut self, x: usize, y: usize, z: isize) {
        use std::cmp::Ordering;
        let mut z = z + self.w[x] - self.w[y];
        let (x, y) = (self.find(x), self.find(y));
        if x == y { return; }

        match self.rank[x].cmp(&self.rank[y]) {
            Ordering::Equal => {
                self.par[y] = x;
                self.rank[x] += 1;
            },
            Ordering::Less => {
                self.par[x] = y;
                z *= -1;
            },
            Ordering::Greater => {
                self.par[y] = x;
            }
        }

        self.w[y] = z;
    }

    fn weight(&mut self, x: usize) -> isize {
        // updating
        self.find(x);
        self.w[x]
    }

    fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    fn find(&mut self, x: usize) -> usize {
        if self.par[x] == x { x }
        else {
            let p = self.par[x];
            let pp = self.find(p);
            self.par[x] = pp;
            self.w[x] += self.w[p];
            self.par[x]
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
