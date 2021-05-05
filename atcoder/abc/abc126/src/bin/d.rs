use std::io;
use std::collections::{ VecDeque, HashMap };


fn main() {
    let n = read_one::<usize>();
    let mut hm = HashMap::new();
    for _ in 1..n {
        let (u, v, w) = {
            let i = read::<usize>();
            (i[0] - 1, i[1] - 1, i[2])
        };
        (*hm.entry(u).or_insert(Vec::new())).push((v, w));
        (*hm.entry(v).or_insert(Vec::new())).push((u, w));
    }

    let mut color = vec![2; n];
    let mut cost = vec![0; n];
    let mut q = VecDeque::new();
    color[0] = 0;
    q.push_back(0);
    while let Some(u) = q.pop_front() {
        for &(v, w) in hm.get(&u).unwrap() {
            if color[v] != 2 { continue; }
            cost[v] = cost[u] + w;
            if cost[v] % 2 == 0 { color[v] = color[u]; }
            else {
                if color[u] == 1 {
                    color[v] = 0;
                } else {
                    color[v] = 1;
                }
            }
            q.push_back(v);
        }
    }

    for i in 0..n {
        println!("{}", color[i]);
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
