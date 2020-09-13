#[allow(unused_imports)]
use std::{io, fmt};

fn main() {
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("read error");
    let num = num.trim().parse::<isize>().expect("parse error");
    for i in 0..num {
        let mut temp = i;
        let mut gen = i;
        while 0<temp{
            gen +=temp%10;
            temp/=10;
        }
    if gen == num {
        println!("{}", i);
        break
    }
    if i == num-1 {
        println!("{}", 0);
    }
    }
}