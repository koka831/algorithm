use std::io;
use std::cmp;
use std::collections::VecDeque;


fn main() {
    let (h, w) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let mut g = Vec::new();
    for _ in 0..h {
        let l = read_one::<String>().chars().collect::<Vec<_>>();
        g.push(l);
    }

    let mut ans = 0;
    if h == 1 && w == 1 { println!("0"); return; }
    for sy in 0..h { for sx in 0..w {
        if g[sy][sx] != '.' { continue; }
        let mut q = VecDeque::new();
        let mut done = vec![vec![false; w]; h];
        let mut cost = vec![vec![1_000_000_000; w]; h];
        let mut min = 1_000_000_000;
        done[sy][sx] = true;
        cost[sy][sx] = 0;
        q.push_back((sy, sx, 1));
        while let Some((y, x, c)) = q.pop_front() {
            let mut p = Vec::new();
            if y > 0 && g[y - 1][x] == '.' { p.push((y - 1, x)); }
            if y < h - 1 && g[y + 1][x] == '.' { p.push((y + 1, x)); }
            if x > 0 && g[y][x - 1] == '.' { p.push((y, x- 1)); }
            if x < w - 1 && g[y][x + 1] == '.' { p.push((y, x + 1)); }
            for (vy, vx) in p {
                if done[vy][vx] { continue; }
                done[vy][vx] = true;
                cost[vy][vx] = cmp::min(cost[vy][vx], c);
                q.push_back((vy, vx, c + 1));
            }
        }
        let mut ret = 0;
        for y in 0..h { for x in 0..w {
            if g[y][x] != '.' { continue; }
            ret = cmp::max(ret, cost[y][x]);
        }}
        ans = cmp::max(ans, ret);
    }}
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
