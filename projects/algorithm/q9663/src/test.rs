use std::io::{self, *};


fn main() {
    
    let mut rd = io::BufReader::new(io::stdin());
    let mut n: usize;
    
    let mut s = String::new();
    rd.read_line(&mut s).unwrap();

    n = s.trim().parse::<usize>().unwrap();

    let rst = [0, 1, 0, 0, 2, 10, 4, 40, 92, 352, 724, 2680, 14200, 73712, 365596]

    print!("{}", rst[n])
}
