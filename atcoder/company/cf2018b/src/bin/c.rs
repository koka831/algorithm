use std::io;


fn main() {
    let n = read_one::<usize>();

    let mut vec = vec![vec!['.'; n]; n];
    let ar = [0, 3, 1, 4, 2];
    let ra = [2, 4, 1, 3, 0];

    for i in 0..n { for j in 0..n {
        let mut flg = false;
        if j % 5 == ar[i % 5] { flg = true; }

        if i == 0 {
            if j % 5 == ra[(j + 3) % 5] { flg = true; }
        }

        if j == 0 {
            if i % 5 == ar[(i + 3) % 5] { flg = true; }
        }

        if i == n - 1 {
            if j % 5 == ar[(i + 1) % 5] { flg = true; }
        }

        if j == n - 1 {
            if i % 5 == ra[j % 5] { flg = true; }
        }

        if flg { vec[i][j] = 'X'; }
    }}

    // let mut cnt = 0;
    for i in 0..n { for j in 0..n {
            // if vec[i][j] == 'X' { cnt += 1; }
            print!("{}", vec[i][j]);
        }
        println!();
    }

    // n == 1000 -> 200,800
    // println!("{}", cnt);
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
