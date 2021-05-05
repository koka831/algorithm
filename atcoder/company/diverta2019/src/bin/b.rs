use std::io;
use std::collections::HashMap;


fn main() {
    let n = read_one::<usize>();
    if n == 1 { println!("1"); return; }
    let mut xy = Vec::new();
    for _ in 0..n {
        let (x, y) = {
            let i = read::<isize>();
            (i[0], i[1])
        };
        xy.push((x, y));
    }

    let mut hm = HashMap::new();
    for i in 0..n { for j in 0..n {
        if i == j { continue; }
        *hm.entry((xy[i].0 - xy[j].0, xy[i].1 - xy[j].1)).or_insert(0) += 1;
    }}
    let ans = hm.iter().max_by_key(|v| v.1).unwrap();
    println!("{}", n - ans.1);
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
