use std::io;
use std::cmp;
use std::collections::VecDeque;

fn main() {
    let n = read_one::<usize>();
    let mut g = vec![Vec::new(); n];
    for i in 0..n - 1 {
        let (a, b) = {
            let i = read::<usize>();
            (i[0] - 1, i[1] - 1)
        };
        g[a].push((i, b));
        g[b].push((i, a));
    }

    let mut colors = vec![0; n - 1];
    let mut q = VecDeque::new();
    let mut siz = 0;
    q.push_back((0, 0, 0, n + 1));
    while let Some((_, u, c, p)) = q.pop_front() {
        let mut color = 1;
        for &v in &g[u] {
            if p == v.1 { continue; }
            if c == color { color += 1; }
            colors[v.0] = color;
            siz = cmp::max(siz, color);
            q.push_back((v.0, v.1, color, u));
            color += 1;
        }
    }
    println!("{}", siz);
    for c in colors {
        println!("{}", c);
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
