#[allow(unused_imports)]
use std::{io, fmt};

macro_rules! read_parse {
    ($($t:ty),*) => ({
        let mut a_str = String::new();
        io::stdin().read_line(&mut a_str).expect("read error");
        let mut a_iter = a_str.split_whitespace();
        (
            $(
                a_iter.next().unwrap().parse::<$t>().expect("parse error"),
            )*
        )
    })
}

fn main() {
    let (_num, max) = read_parse!(isize, isize);
    let mut card: Vec<isize> = Vec::new();
    let mut card_str = String::new();
    io::stdin().read_line(&mut card_str).expect("card error");
    let card_iter = card_str.split_whitespace();
    for i in card_iter {
        let a= i.trim().parse::<isize>().expect("e");
        card.push(a);
    }
    let mut rst: isize =0;
    for i in 0..card.len() {
        for j in i+1..card.len() {
            for k in j+1..card.len(){
                if card[i]+card[j]+card[k]<= max && card[i]+card[j]+card[k]> rst{
                    rst = card[i]+card[j]+card[k];
                }
            }
        }
    }
    println!("{}", rst);
}