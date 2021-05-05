use std::io;


fn main() {
    let (n, m, q) = {
        let i = read::<usize>();
        (i[0], i[1], i[2])
    };

    let mut cnt = vec![vec![0; n]; n];
    for _ in 0..m {
        let i = read::<usize>();
        let (l, r) = (i[0] - 1, i[1] - 1);
        for i in l..(r + 1) {
            for j in i..r {
                cnt[i][j] += 1;
            }
        }
    }

    let mut pq = Vec::new();
    for _ in 0..q {
        let i = read::<usize>();
        pq.push((i[0] - 1, i[1] - 1));
    }

    /*
    let mut ans = vec![vec![0; n]; n];
    for i in 0..n {
        for j in i..n {
            println!("{}:{}", i, j);
            if i == j { ans[i][j] = cnt[i][j]; }
            else { ans[i][j] = ans[i][j - 1] + cnt[i][j] + cnt[j][j]; }
        }
    }
    */


    for q in pq {
        println!("{}", cnt[q.0][q.1]);
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
