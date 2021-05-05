use std::io;
use std::usize;


fn main() {
    let (n, k) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let an = read::<usize>();

    if n == 1 { println!("{}", an[0]); return; }
    let mut count = vec![0; 80];

    for a in an {
        let x = format!("{:080b}", a).chars().collect::<Vec<char>>();
        for i in 0..80 {
            count[i] += if x[i] == '1' { 1 } else { 0 };
        }
    }

    let mut ans = String::new();

    for i in 0..80 {
        if count[i] as f64 >= n as f64 / 2.0 { ans += "0"; } else { ans += "1"; }
    }

    let mut _k: String = format!("{:b}", k);
    let mut prev = "0";
    for i in 2..80 {
        let tmp = &ans[ans.len() - i..ans.len()];
        if usize::from_str_radix(&_k, 2).unwrap() < usize::from_str_radix(&tmp, 2).unwrap() {
            break;
        }
        prev = tmp;
    }

    let mut ans = 0;
    let prev = prev.chars().collect::<Vec<char>>();
    for i in 0..prev.len() {
        if prev[prev.len() - i - 1] == '0' {
            let add= 2usize.pow(i as u32) * count[count.len() - i - 1];
            ans += add;
        } else {
            let add = 2usize.pow(i as u32) * (n - count[count.len() - i - 1]);
            ans += add;
        }
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
