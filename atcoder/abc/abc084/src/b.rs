use std::io;


fn main() {

    let mut buf = String::new();

    io::stdin().read_line(&mut buf).unwrap();
    let vec = buf.split(" ")
        .map(|c| c.to_string())
        .collect::<Vec<String>>();

    println!("{:?}", vec);
}
