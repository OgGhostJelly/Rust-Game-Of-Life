mod game_context;
mod grid;
mod position;

use game_context::GameContext;
use grid::Grid;
use position::{Position2D, ToPosition2D};
use std::{time::Instant, collections::HashSet};
use sdl2::{self, keyboard::Keycode, pixels::Color, rect::Rect};
use rayon::prelude::*;

fn main() {
    // Initialize the grid
    const GRID_SIZE: [usize; 2] = [250, 250];
    let mut grid = Grid::new([[false; GRID_SIZE[1]]; GRID_SIZE[0]]);

    // Set the rows to random stuff
    use rand::Rng;
    for row in &mut grid {
        for tile in row {
            *tile = rand::thread_rng().gen_bool(0.5);
        }
    }

    // Set a 20x20 area at the origin to empty
    for y in 0..20 {
        for x in 0..20 {
            grid[y][x] = false;
        }
    }

    // Spawn in a glider
    grid[0][0] = true;
    grid[2][0] = true;
    grid[1][1] = true;
    grid[1][2] = true;
    grid[2][1] = true;

    // Setup
    let mut c = GameContext::setup("Conway's Game Of Life - Rust SDL2", 800, 600);
    let timestep_length_secs = 0.1;
    let mut timestep = Instant::now();
    let mut camera_position: Position2D<f64> = [0.0, 0.0].into();
    let mut scale = [1.0, 1.0].to_pos2();
    let mut queued_updates = HashSet::<(usize, usize)>::new();

    for (y, row) in grid.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            match value {
                true => {
                    queued_updates.insert((y, x));

                    for neighbour in grid.get_8_neighbours(y, x) {
                        match neighbour {
                            Some((pos, _)) => { queued_updates.insert(pos); },
                            None => (),
                        }
                    }
                },
                false => (),
            }
        }
    }

    // Register ui input actions
    c.register_actions([
        ("up", [Keycode::W, Keycode::Up]),
        ("left", [Keycode::A, Keycode::Left]),
        ("down", [Keycode::S, Keycode::Down]),
        ("right", [Keycode::D, Keycode::Right]),
        ("zoom_in", [Keycode::I, Keycode::Z]),
        ("zoom_out", [Keycode::O, Keycode::X]),
    ]);
    
    c.tick(|c| {
        // Draw the background
        c.canvas.set_draw_color(Color::BLACK);
        c.canvas.clear();

        // Move the camera
        if c.is_action_down("up") {
            camera_position += [0.0, -750.0].to_pos2() * c.delta.elapsed().as_secs_f64();
        }

        if c.is_action_down("left") {
            camera_position += [-750.0, 0.0].to_pos2() * c.delta.elapsed().as_secs_f64()
        }

        if c.is_action_down("down") {
            camera_position += [0.0, 750.0].to_pos2() * c.delta.elapsed().as_secs_f64()
        }

        if c.is_action_down("right") {
            camera_position += [750.0, 0.0].to_pos2() * c.delta.elapsed().as_secs_f64()
        }

        if c.is_action_down("zoom_in") {
            scale += [1.0, 1.0].to_pos2() * c.delta.elapsed().as_secs_f64()
        }

        if c.is_action_down("zoom_out") {
            scale -= [1.0, 1.0].to_pos2() * c.delta.elapsed().as_secs_f64()
        }

        println!("({}, {})", scale.x(), scale.y());

        c.canvas.set_draw_color(Color::GREY);
        _ = c.canvas.draw_rect(Rect::new(
            -camera_position.x() as i32 - 25, 
            -camera_position.y() as i32 - 25, 
            ((50.0 * scale.x() * 250.0) + 25.0) as u32,
            ((50.0 * scale.y() * 250.0) + 25.0) as u32,
        ));

        let draws: Vec<_> = (&grid).into_iter().enumerate().par_bridge().map(|(y, row)| {
            row.iter().enumerate().par_bridge().filter_map(move |(x, tile)| {
                match tile {
                    true => (),
                    false => return None,
                }
                
                let size = [50.0, 50.0].to_pos2();
                let position = <Position2D<usize> as Into<Position2D<f64>>>::into([x, y].to_pos2()) * size * scale;
                let draw_position = position - camera_position.clone();

                if draw_position.x() > 850.0 || draw_position.y() > 650.0 {
                    return None;
                }

                let rect_size: Position2D<u32> = (size * scale).into();

                Some((Color::WHITE, Rect::from_center(
                    draw_position,
                    rect_size.x() + 1,
                    rect_size.y() + 1,
                )))
            })
        }).flatten().collect();

        for (color, rect) in draws.iter() {
            c.canvas.set_draw_color(*color);
            let _ = c.canvas.fill_rect(*rect);
        }

        // time step/simulate game of life
        if timestep.elapsed().as_secs_f64() >= timestep_length_secs {
            let processing_timer = Instant::now();
            let updates_count = queued_updates.len();

            let changes: Vec<_> = queued_updates.par_iter().map(|(y, x)| {
                let (y, x) = (*y, *x);
                
                let neighbour_count = grid.get_8_neighbours(y, x).iter().filter(|value| match value {
                    Some((_, value)) => **value,                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  
                    None => false,
                }).count();

                let value = match grid[y][x] {
                    // Any live cell with fewer than two live neighbours dies, as if by underpopulation.
                    true if neighbour_count < 2 => false,
                    // Any live cell with more than three live neighbours dies, as if by overpopulation.
                    true if neighbour_count > 3 => false,
                    // Any live cell with two or three live neighbours lives on to the next generation.
                    true => true,
                    // Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
                    false if neighbour_count == 3 => true,
                    // Any dead cell without exactly three live neighbours stays dead, as if by being dead.
                    false => false,
                };

                if grid[y][x] != value {
                    Some(((y, x), value))
                } else {
                    None
                }
            }).filter(|value| {
                match value {
                    Some(_) => true,
                    None => false,
                }
            }).map(|value| {
                match value {
                    Some(value) => value,
                    None => unreachable!("We filtered out all `None` values so this shouldn't be possible"),
                }
            }).collect();

            queued_updates.clear();

            for ((y, x), value) in changes {
                for neighbour in grid.get_8_neighbours(y, x) {
                    match neighbour {
                        Some(((y, x), _)) => { queued_updates.insert((y, x)); },
                        None => (),
                    }
                }

                grid[y][x] = value;
            }

            println!("Simulation updated {} tiles in {}s.", updates_count, processing_timer.elapsed().as_secs_f64());

            timestep = Instant::now();
        }

        c.canvas.present();
    })
}
