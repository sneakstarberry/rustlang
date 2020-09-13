use std::io::Write;
use std::io::BufReader;
use std::io::BufRead;



fn main() {
    let mut rd = BufReader::new(std::io::stdin());
    let stdin = std::str::from_utf8;
    let mut input = Vec::new();
    rd.read_until(b'\n', &mut input).unwrap();
    let n: i32 = stdin(&mut input).unwrap()
        .trim().parse().unwrap();
    let mut b = [0; 10001];
    input.clear();
    for _ in 0..n {
        rd.read_until(b'\n',&mut input).unwrap();
        b[stdin(&mut input).unwrap()
            .trim().parse::<usize>().unwrap()]+=1;
        input.clear();
    }

    for i in 1..10001 {
        input.clear();
        if b[i]!=0{
            for _ in 0..b[i] {
                write!(input,"{}\n", i).unwrap();
            }
            print!("{}", stdin(&input).unwrap());
        }
    }
}

// fn to_int (buf: Vec<u8>) -> usize {
//     let mut n: usize = 0;
//     for i in buf[..buf.len()-2].iter().enumerate(){
//         n = n*10 + (*(i.1)-b'0') as usize;
//     }
//     return n
// }

// fn main() {
//     let mut rd = BufReader::new(std::io::stdin());
//     let mut v = Vec::new();

//     rd.read_until(b'\n', &mut v).unwrap();
//     let n: usize = to_int(v.clone());
//     let mut b = [0; 10001];
//     v.clear();

//     for _ in 0..n {
//         rd.read_until(b'\n', &mut v).unwrap();
//         b[to_int(v.clone())]+=1;
//         v.clear();
//     }

//     for i in 1..10001 {
//         if b[i]!=0 {
//             for _ in 0..b[i] {
//                 write!(v, "{}\n", i).unwrap();
//             }
//             print!("{}", std::str::from_utf8(&v).unwrap());
//             v.clear();
//         }
//     }
// }