use std::io::{self, *};

static mut RC: [[usize; 15]; 15] = [[0; 15]; 15];
static mut CNT: usize = 0;
static mut N: usize = 0;

fn main() {
    unsafe {
        let mut rd = io::BufReader::new(io::stdin());

        let mut s = String::new();
        rd.read_line(&mut s).unwrap();

        N = s.trim().parse::<usize>().unwrap();

        for i in 0..N {
            n_queen(0, i)
        }
        s.clear();
        print!("{}", CNT)
    }

    fn n_queen(row: usize, col: usize) {
        unsafe {
            if row == N - 1 {
                CNT = CNT + 1;
                print!("{}", CNT);
                return;
            }

            for i in (row + 1)..N {
                RC[i][col] += 1;
                let j = i - row;
                if col as isize - j as isize >= 0 {
                    RC[i][col - j] += 1;
                }
                if col + j < N {
                    RC[i][col + j] += 1;
                }
            }

            for i in 0..N {
                if RC[row + 1][i] == 0 {
                    n_queen(row + 1, i)
                }
            }
            for i in (row + 1)..N {
                RC[i][col] -= 1;
                let j = i - row;
                if col as isize - j as isize >= 0 {
                    RC[i][col - j] -= 1;
                }
                if col + j < N {
                    RC[i][col + j] -= 1;
                }
            }
        }
    }
}
