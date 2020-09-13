#![allow(unused)]
use std::{
    error::Error,
    io::{self, BufRead, Read, Write},
};
fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    let mut buff = Vec::new();
    let mut stdin = io::stdin();

    stdin.read_line(&mut buffer)?;
    io::stdin().lock().read_until(b'\n', &mut buff)?;
    io::stdout().write_all(&buff)?;
    io::stdout().write_all(&buff)?;
    println!("{}", buffer);
    io::stdout().flush();
    Ok(())
}
