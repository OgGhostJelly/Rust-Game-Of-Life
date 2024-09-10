use std::{io::stdin, time::Instant};

use board::Board;
use rand::thread_rng;

mod board;

fn main() {
    let mut board = Board::rand(&mut thread_rng(), 0.5);
    
    loop {
        let cells = board.cells();
        
        /*{
            let lock = stdout().lock();
            
            for row in cells {
                for cell in row {
                    if cell.is_alive() {
                        print!("#")
                    } else {
                        print!(".")
                    }
                }
        
                println!()
            }

            drop(lock)
        }*/

        let _ = stdin().read_line(&mut String::new());

        let previous = Instant::now();
        board = board.tick();
        println!("Generated in {:?}", (Instant::now() - previous).as_millis());
    }
}