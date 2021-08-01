fn main() {
    let n = io::read_one::<usize>();
    let mut g = vec![vec![]; n];

    for _ in 1..n {
        let (a, b) = {
            let i = io::read::<usize>();
            (i[0] - 1, i[1] - 1)
        };
        g[a].push(b);
        g[b].push(a);
    }


    let (_, node) = dfs(&g, 0);
    // calculate diameter
    let (length, _) = dfs(&g, node);

    println!("{}", length + 1);
}

// returns (longest_path_length, id of longest_path_node)
fn dfs(g: &Vec<Vec<usize>>, s: usize) -> (usize, usize) {
    let mut q = Vec::new();
    let mut arrived = vec![false; g.len()];
    let (mut length, mut node) = (0, 0);
    // (current, length)
    q.push((s, 0));

    while let Some((c, l)) = q.pop() {
        arrived[c] = true;

        if length < l {
            node = c;
            length = l
        }

        for &u in &g[c] {
            if arrived[u] { continue; }
            q.push((u, l + 1));
        }
    }

    (length, node)
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
