#![allow(unused)]
use std::{
    error::Error,
    io::{self, BufRead, BufReader, Read, Write},
};
fn main() -> Result<(), Box<dyn Error>> {
    let mut t = String::new();
    let mut tmp = String::new();
    let mut m: usize;
    let mut buff = Vec::new();
    let mut stdin = io::stdin();

    stdin.lock().read_line(&mut t)?;
    let mut t = t.trim().parse::<usize>()?;

    let mut arr: Vec<usize> = vec![0; 100001];
    arr[1] = 1;

    let mut min = vec![0; 2];
    let mut now = vec![0; 2];
    for k in 0..t {
        min[0] = 0;
        min[1] = 1000001;
        let mut big: usize;
        tmp.clear();
        std::io::stdin().read_line(&mut tmp)?;
        m = tmp.trim().parse::<usize>()?;
        for i in 2..=m {
            if arr[i] == 1 {
                continue;
            }
            let mut j = i * i;
            while j <= m {
                arr[j] = 1;
                j += i;
            }
        }
        for i in 2..=m / 2 {
            if arr[i] != 1 && arr[m - i] != 1 {
                now[0] = i;
                now[1] = m - i;
                if now[1] - now[0] <= min[1] - min[0] {
                    min[0] = now[0];
                    min[1] = now[1];
                }
            }
        }
        for i in 0..2 {
            write!(buff, "{} ", min[i]);
        }
        write!(buff, "\n");
    }

    io::stdout().write_all(&buff)?;
    io::stdout().flush();
    Ok(())
}
