mod game_context;
mod grid;
mod position;

use game_context::GameContext;
use grid::Grid;
use position::{Position2D, ToPosition2D};
use std::time::Instant;
use sdl2::{self, keyboard::Keycode, pixels::Color, rect::Rect};
use rand::Rng;

fn main() {
    // Initialize the grid
    const GRID_SIZE: [usize; 2] = [200, 200];
    let mut grid = Grid::new([[false; GRID_SIZE[1]]; GRID_SIZE[0]]);

    // Set the rows to random stuff
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

    // Spawn acorn
    //grid[0+15][1+15] = true;
    //
    //grid[2+15][1+15] = true;
    //grid[2+15][0+15] = true;
    //
    //grid[1+15][3+15] = true;
    //
    //grid[2+15][4+15] = true;
    //grid[2+15][5+15] = true;
    //grid[2+15][6+15] = true;

    // Setup
    let mut c = GameContext::setup("Conway's Game Of Life - Rust SDL2");
    let timestep_length_secs = 0.1;
    let mut timestep = Instant::now();
    let mut camera_position: Position2D<f64> = [0.0, 0.0].into();
    let mut scale = [1.0, 1.0].to_pos2();
    let mut next_update = Vec::new();

    for (y, row) in grid.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            next_update.push((y, x))
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
        c.canvas.set_draw_color(Color::RGB(125, 125 ,125));
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

        // Draw the grid
        for (y, row) in (&grid).into_iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                c.canvas.set_draw_color(match tile {
                    true => Color::WHITE,
                    false => Color::BLACK,
                });

                let size = [50.0, 50.0].to_pos2();
                //let offset = [25.0, 25.0].to_pos2();
                let pos: Position2D<f64> = [x, y].to_pos2().into();

                let rect_size: Position2D<u32> = (size * scale).into();
                
                let position = pos * (size/* + offset*/) * scale; // [x as f64 * 75.0, y as f64 * 75.0].to_pos2() * 1.0;
                let _ = c.canvas.fill_rect(Rect::from_center(
                    position - camera_position.clone(),
                    rect_size.x() + 1,
                    rect_size.y() + 1,
                ));
            }
        }

        // time step/simulate game of life
        if timestep.elapsed().as_secs_f64() >= timestep_length_secs {
            let mut changes: Vec<((usize, usize), bool)> = Vec::new();

            for (y, row) in grid.iter().enumerate() {
                for (x, tile) in row.iter().enumerate() {
                    let neighbour_count = grid.get_8_neighbours(y, x).iter().filter(|(pos, value)| match value {
                        Some(value) => **value,
                        None => false,
                    }).count();

                    let value = match tile {
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
                        changes.push(((y, x), value));
                    }
                }
            }

            for ((y, x), value) in changes {
                grid[y][x] = value;
            }

            timestep = Instant::now();
        }

        c.canvas.present();
    })
}
