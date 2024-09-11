use std::{sync::Arc, time::{Duration, Instant}};

use board::Board;
use colored::Colorize;
use game_context::RenderContext;
use rand::thread_rng;
use sdl2::event::Event;

mod board;
mod game_context;

fn main() -> Result<(), String> {
    #[cfg(debug_assertions)]
    eprintln!("{}", "WARN: you are running in debug. debug is slower and less optimized than release. to run in release do `cargo run --release`".yellow());
    
    // Initialize SDL2

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let mut canvas = video_subsystem.window("The Game of Life - Rust", 480, 480)
        .resizable()
        .build()
        .expect("could not initialize video subsystem")
        .into_canvas()
        .build()
        .expect("could not make a canvas");

    // Initialize Game

    let mut board = Board::<1000, 1000>::rand(&mut thread_rng(), 0.5);
    let draw_board = Arc::new(std::sync::Mutex::new(board.clone()));
    let draw_board_ref = draw_board.clone();

    std::thread::spawn(move || {
        loop {
            let old_board = board;
            board = old_board.tick();
            
            let mut cached_board = draw_board_ref.lock().unwrap();
            *cached_board = old_board;
            drop(cached_board);

            ::std::thread::sleep(Duration::from_nanos(16000000));
        }
    });

    let mut game_context = RenderContext::new();
    let mut event_pump = sdl_context.event_pump()?;
    let mut last_update: Instant = Instant::now();

    // Main event loop

    'main: loop {
        let delta = Instant::now() - last_update;
        last_update = Instant::now();

        for event in event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                break 'main;
            }

            game_context.handle_event(event);
        }
        
        game_context = game_context.tick(delta);

        canvas.set_draw_color(game_context.background_color());
        canvas.clear();

        let board = draw_board.lock().unwrap();
        game_context.draw(&*board, &mut canvas);
        drop(board);

        canvas.present();
    }

    Ok(())
}