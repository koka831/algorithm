use std::io;


fn main() {
    let (n, q) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let mut uf = UnionFind::new(n);

    for _ in 0..q {
        let (com, x, y) = {
            let i = read::<usize>();
            (i[0], i[1], i[2])
        };
        if com == 0 {
            uf.unite(x, y);
        } else {
            let res = uf.same(x, y);
            if res { println!("1"); }
            else { println!("0"); }
        }
    }
}

struct UnionFind {
    par: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind { par: (0..n).collect(), }
    }

    fn unite(&mut self, x: usize, y: usize) {
        let px = self.find(x);
        let py = self.find(y);
        if px == py { return; }
        self.par[py] = px;
    }

    fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    fn find(&mut self, x: usize) -> usize {
        if self.par[x] == x { x }
        else {
            let p = self.par[x];
            self.par[x] = self.find(p);
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
