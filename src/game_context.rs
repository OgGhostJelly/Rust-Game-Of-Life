use std::{collections::HashSet, time::Duration};

use colored::Colorize;
use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::{FPoint, FRect},
    render::WindowCanvas,
};

use crate::board;

pub trait GameOfLifeBoard {
    fn alive_cells(&self) -> impl Iterator<Item = (usize, usize)>;
    fn width(&self) -> usize;
    fn height(&self) -> usize;
}

impl<const WIDTH: usize, const HEIGHT: usize> GameOfLifeBoard for board::Board<WIDTH, HEIGHT> {
    fn alive_cells(&self) -> impl Iterator<Item = (usize, usize)> {
        self.alive_cells().iter().cloned()
    }

    fn width(&self) -> usize {
        WIDTH
    }

    fn height(&self) -> usize {
        HEIGHT
    }
}

pub struct RenderContext {
    camera: Camera,
    key_presses: HashSet<Keycode>,
}

impl RenderContext {
    pub fn new() -> Self {
        Self {
            camera: Camera::default(),
            key_presses: HashSet::default(),
        }
    }

    pub fn tick(mut self, delta: Duration) -> Self {
        self.move_camera(delta);
        self.scale_camera(delta);
        self
    }

    pub fn scale_camera(&mut self, delta: Duration) {
        let delta = delta.as_secs_f32();
        let mut by = 0.0;

        if self.is_key_pressed(&Keycode::I) {
            by += 1.0;
        }
        if self.is_key_pressed(&Keycode::O) {
            by -= 1.0;
        }

        self.camera.scale += by * 0.9 * delta;
        self.camera.scale = self.camera.scale.clamp(-1.5, 5.0);
    }

    pub fn move_camera(&mut self, delta: Duration) {
        let delta = delta.as_secs_f32();
        let mut by = (0.0, 0.0);

        if self.any_key_pressed(&[&Keycode::Left, &Keycode::A]) {
            by.0 -= 1.0;
        }
        if self.any_key_pressed(&[&Keycode::Right, &Keycode::D]) {
            by.0 += 1.0;
        }
        if self.any_key_pressed(&[&Keycode::Up, &Keycode::W]) {
            by.1 -= 1.0;
        }
        if self.any_key_pressed(&[&Keycode::Down, &Keycode::S]) {
            by.1 += 1.0;
        }

        let by = (by.0 * delta * 48.0, by.1 * delta * 48.0);

        self.camera.move_by(by);
    }

    pub fn is_key_pressed(&self, keycode: &Keycode) -> bool {
        self.key_presses.contains(keycode)
    }

    pub fn any_key_pressed(&self, keycodes: &[&Keycode]) -> bool {
        keycodes
            .into_iter()
            .any(|keycode| self.is_key_pressed(keycode))
    }

    pub fn handle_event(&mut self, event: Event) {
        match event {
            Event::KeyDown {
                keycode, keymod, ..
            } if keymod.is_empty() => match keycode {
                Some(keycode) => {
                    self.key_presses.insert(keycode);
                }
                None => (),
            },
            Event::KeyUp {
                keycode, keymod, ..
            } if keymod.is_empty() => match keycode {
                Some(keycode) => {
                    self.key_presses.remove(&keycode);
                }
                None => (),
            },
            _ => (),
        }
    }

    pub fn world_to_screen<B: GameOfLifeBoard>(
        &self,
        board: &B,
        canvas: &WindowCanvas,
        x: f32,
        y: f32,
    ) -> (f32, f32) {
        let center_grid_x = board.width() as f32 / 2.0;
        let center_grid_y = board.height() as f32 / 2.0;

        let (center_canvas_x, center_canvas_y) = match canvas.output_size() {
            Ok((x, y)) => (x, y),
            Err(e) => {
                eprintln!(
                    "{}",
                    format!("WARN: error when getting `canvas.output_size`: {e}").yellow()
                );
                return (0.0, 0.0);
            }
        };

        let scale = 2.0f32.powf(self.camera.scale);
        let center_canvas_x = center_canvas_x as f32 / 2.0 / scale;
        let center_canvas_y = center_canvas_y as f32 / 2.0 / scale;

        let (tile_x, tile_y) = (x as f32, y as f32);
        let (camera_x, camera_y) = self.camera.position;
        (
            tile_x - camera_x + center_canvas_x - center_grid_x,
            tile_y - camera_y + center_canvas_y - center_grid_y,
        )
    }

    pub fn tile_to_screen_rect<B: GameOfLifeBoard>(
        &self,
        board: &B,
        canvas: &WindowCanvas,
        x: usize,
        y: usize,
    ) -> FRect {
        let (x, y) = self.world_to_screen(board, canvas, x as f32, y as f32);
        FRect::new(x, y, 1.0, 1.0)
    }

    pub fn tile_to_screen_point<B: GameOfLifeBoard>(
        &self,
        board: &B,
        canvas: &WindowCanvas,
        x: usize,
        y: usize,
    ) -> FPoint {
        let (x, y) = self.world_to_screen(board, canvas, x as f32, y as f32);
        FPoint::new(x, y)
    }

    pub fn draw_border<B: GameOfLifeBoard>(&self, board: &B, canvas: &mut WindowCanvas) {
        let top_left = self.tile_to_screen_point(board, canvas, 0, 0);
        let bottom_right = self.tile_to_screen_point(board, canvas, board.width(), board.height());
        let top_right = FPoint::new(bottom_right.x, top_left.y);
        let bottom_left = FPoint::new(top_left.x, bottom_right.y);

        canvas.set_draw_color(Color::WHITE);
        let _ = canvas
            .draw_flines([top_left, top_right, bottom_right, bottom_left, top_left].as_slice());
    }

    pub fn draw<B: GameOfLifeBoard>(&self, board: &B, canvas: &mut WindowCanvas) {
        let scale = 2.0f32.powf(self.camera.scale);
        let _ = canvas.set_scale(scale, scale);

        self.draw_border(board, canvas);

        for (x, y) in board.alive_cells() {
            canvas.set_draw_color(Color::WHITE);
            let ret = canvas.draw_frect(self.tile_to_screen_rect(board, canvas, x, y));
            #[cfg(debug_assertions)]
            if let Err(e) = ret {
                eprintln!(
                    "{}",
                    format!("WARN: failed to draw tile ({x}, {y}): {e}").yellow()
                )
            }
        }
    }

    pub fn background_color(&self) -> Color {
        Color::RGB(10, 10, 20)
    }
}

pub struct Camera {
    pub position: (f32, f32),
    pub scale: f32,
}

impl Camera {
    pub fn move_by(&mut self, by: (f32, f32)) {
        self.position.0 += by.0;
        self.position.1 += by.1;
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: (0.0, 0.0),
            scale: 0.0,
        }
    }
}
