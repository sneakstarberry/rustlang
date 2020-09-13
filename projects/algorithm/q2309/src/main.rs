use std::io::{self, *};

static mut V: Vec<u8> = Vec::new();

fn main() {
    let mut rd = io::BufReader::new(io::stdin());
    let mut input = String::new();
    let mut dwarfs: Vec<usize> = Vec::with_capacity(9);
    let sevens: Vec<usize> = vec![0; 2];
    let visit: Vec<bool> = vec![false; 9];
    for _ in 0..9 {
        rd.read_line(&mut input).unwrap();
        dwarfs.push(input.trim().parse::<usize>().unwrap());
        input.clear();
    }
    dwarfs.sort();
    snow_white(0, dwarfs, sevens, visit);
}

fn snow_white(cnt: usize, dwarfs: Vec<usize>, mut sevens: Vec<usize>, mut visit: Vec<bool>) {
    let mut wr = BufWriter::new(std::io::stdout());
    let total_sum = dwarfs.iter().sum::<usize>();
    let sum = sevens.iter().sum::<usize>();

    if cnt == 2 && total_sum - sum == 100 && sevens.len() == 2 {
        sevens.sort();
        unsafe {
            V.clear();
        }
        for v in dwarfs.iter() {
            if *v != sevens[0] && *v != sevens[1] {
                unsafe {
                    write!(V, "{}\n", v).unwrap();
                }
            }
        }
        unsafe {
            wr.write_all(&V).unwrap();
        }
        wr.flush().unwrap();
        std::process::exit(0);
        
    }
    for i in 0..9 {
        if visit[i] != true {
            visit[i] = true;
            if sum < 100 && cnt < 2 {
                sevens[cnt] = dwarfs[i];
                snow_white(cnt + 1, dwarfs.clone(), sevens.clone(), visit.clone());
            }
            visit[i] = false;
        }
    }
}
