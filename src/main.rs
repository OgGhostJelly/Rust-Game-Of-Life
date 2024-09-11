use std::{io::stdin, time::Instant};

use board::Board;
use colored::Colorize;
use rand::thread_rng;

mod board;

fn main() {
    #[cfg(debug_assertions)]
    eprintln!("{}", "WARN: you are running in debug. debug is slower and less optimized than release. to run in release do `cargo run --release`".yellow());
    
    let mut board: Board::<250, 250> = Board::rand(&mut thread_rng(), 0.5);

    loop {
        let _ = stdin().read_line(&mut String::new());
        let previous = Instant::now();
        board = board.tick();
        println!("Generated in {:?}", (Instant::now() - previous).as_millis());
    }
}