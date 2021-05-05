use std::cmp::Ordering;
use cargo_snippet::snippet;

/// UnionFind
#[snippet("UnionFind")]
pub struct UnionFind {
    par: Vec<usize>,
    rank: Vec<usize>,
}

#[snippet("UnionFind")]
impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            par: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    // return an id of the parent node
    pub fn find(&mut self, x: usize) -> usize {
        if self.par[x] == x { return x; }

        let y = self.par[x];
        self.par[x] = self.find(y);
        self.par[x]
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    pub fn unite(&mut self, x: usize, y: usize) {
        let x = self.find(x);
        let y = self.find(y);

        if x == y { return; }

        match self.rank[x].cmp(&self.rank[y]) {
            Ordering::Less => {
                self.par[x] = y;
            },
            Ordering::Equal => {
                self.par[y] = x;
                self.rank[x] += 1;
            },
            Ordering::Greater => {
                self.par[y] = x;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_union_find() {
        // 0 - 1 - 2
        //     |
        //     +- 3
        //
        // 4 - 5
        let n = 6;
        let g = vec![vec![1], vec![2, 3], vec![1], vec![1], vec![5], vec![4]];
        let mut uf = UnionFind::new(n);
        for i in 0..n { for &u in g[i].iter() {
            uf.unite(i, u);
        }}

        assert!(uf.same(0, 1));
        assert!(uf.same(0, 2));
        assert!(uf.same(0, 3));
        assert!(!uf.same(0, 4));
    }
}
