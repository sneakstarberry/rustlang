use std::io::Write;
use std::io::BufWriter;
use std::io::BufReader;
use std::io::BufRead;
// use std::time::Instant;

static mut V: Vec<u8> = Vec::new();

fn cn(cnt: usize, n: usize, m:usize, mut visit: Vec<bool>, mut arr: Vec<usize>) {
    unsafe {
        if cnt == m {
            
            for i in arr.iter() {
                write!(V, "{} ", i + 1).unwrap();
            }
            write!(V, "\n").unwrap();
           return
        }
        for i in 0..n {
            if visit[i] != true {
                visit[i] = true;
                arr[cnt] = i;
                cn(cnt + 1, n, m, visit.clone(), arr.clone());
                visit[i] = false;
            }
        }
    }
}

fn main(){
    // let start = Instant::now();
    let mut rd = BufReader::new(std::io::stdin());
    let mut wr = BufWriter::new(std::io::stdout());
    let mut s = String::new();
    rd.read_line(&mut s).unwrap();
    let mut s = s.split_whitespace();
    let n = s.next().unwrap().parse::<usize>().unwrap();
    let m = s.next().unwrap().parse::<usize>().unwrap();
    let visit = vec![false; n+1];
    let arr = vec![0; m];
    
    cn(0, n, m, visit, arr);

    unsafe{
        wr.write_all(&V).unwrap();
    }
    wr.flush().unwrap();
    // let elapsed = start.elapsed();
    // println!("Millis: {} ms", elapsed.as_millis());
}
