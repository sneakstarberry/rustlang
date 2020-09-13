
use std::io::BufReader;
use std::io::BufRead;



fn main() {
    let array = ["c=", "c-", "dz=", "d-", "lj", "nj", "s=", "z="];
    let mut alphabat = String::new();
    let mut rd = BufReader::new(std::io::stdin());
    rd.read_line(&mut alphabat).unwrap();

    for i in array.iter() {
        alphabat = alphabat.replace(i, "1");
    }

    println!("{}", alphabat.trim().len());
}
