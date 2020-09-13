#![allow(unused)]
use std::{
    error::Error,
    io::{self, BufRead, Read, Write},
};
fn main() -> Result<(), Box<dyn Error>> {
    let mut mn = String::new();
    let mut m: usize;
    let mut n: usize;
    let mut buff = Vec::new();
    
    let mut stdin = io::stdin();

    stdin.lock().read_line(&mut mn)?;
    let mut mn = mn.split_whitespace();
    m = mn.next().unwrap().parse::<usize>()?;
    n = mn.next().unwrap().parse::<usize>()?;

    let mut arr : Vec<usize> = vec![0; 1000001];
    arr[1] = 1;



    for i in 2..=n{
        if arr[i]==1{
            continue;
        }
        let mut j = i+i;
        while j<=n{
            arr[j] = 1;
            j+=i;
        }
    }
    for i in m..=n{
        if arr[i] != 1 {
            write!(buff, "{}\n", i);
        }
    }
    io::stdout().write_all(&buff)?;
    io::stdout().flush();
    Ok(())
}
