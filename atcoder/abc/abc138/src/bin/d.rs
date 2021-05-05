use std::io;
use std::collections::VecDeque;


fn main() {
    let (n, q) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    let mut node = vec![Vec::new(); n];
    let mut score = vec![0; n];

    for _ in 0..n - 1 {
        let (a, b) = {
            let i = read::<usize>();
            (i[0] - 1, i[1] - 1)
        };
        node[a].push(b);
        node[b].push(a);
    }

    for _ in 0..q {
        let (p, x) = {
            let i = read::<usize>();
            (i[0] - 1, i[1])
        };
        score[p] += x;
    }

    let mut que = VecDeque::new();
    que.push_back((0, 0));
    while let Some((p, v)) = que.pop_front() {
        for &c in node[v].iter() {
            if c == p { continue; }
            // println!("score[{}] += {}", c, score[v]);
            score[c] += score[v];
            que.push_back((v, c));
        }
    }
    let ans = score.iter()
        .map(|&i| i.to_string())
        .collect::<Vec<String>>()
        .join(" ");
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
