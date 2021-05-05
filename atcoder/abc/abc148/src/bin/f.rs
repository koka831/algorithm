use std::io;
use std::cmp;

fn main() {
    let (n, u, v) = {
        let i = read::<usize>();
        (i[0], i[1] - 1, i[2] - 1)
    };
    let mut g = vec![Vec::new(); n];
    for _ in 1..n {
        let (a, b) = {
            let i = read::<usize>();
            (i[0] - 1, i[1] - 1)
        };
        g[a].push(b);
        g[b].push(a);
    }
    let mut du = vec![0; n];
    let mut qu = Vec::new();
    qu.push((u, n + 1));
    while let Some((p, par)) = qu.pop() {
        for &x in &g[p] {
            if x == par { continue; }
            du[x] = du[p] + 1;
            qu.push((x, p));
        }
    }

    let mut dv = vec![0; n];
    let mut qv = Vec::new();
    qv.push((v, n + 1));
    while let Some((p, par)) = qv.pop() {
        for &x in &g[p] {
            if x == par { continue; }
            dv[x] = dv[p] + 1;
            qv.push((x, p));
        }
    }

    let mut ans = 0;
    for i in 0..n {
        if dv[i] < du[i] { continue; }
        ans = cmp::max(ans, dv[i] - 1);
    }
    println!("{}", ans);
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
