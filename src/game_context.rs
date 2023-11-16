use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use sdl2::{
    self, event::Event, keyboard::Keycode, render::Canvas, video::Window, EventPump, Sdl,
    VideoSubsystem,
};

/// A UI input action.
pub struct InputAction {
    keys: Vec<Keycode>,
}

impl InputAction {
    pub fn new<T: Into<Vec<Keycode>>>(keys: T) -> Self {
        Self { keys: keys.into() }
    }
}

/// An sdl2 helper that handles some simple methods.
pub struct GameContext<'a> {
    pub sdl_context: Sdl,
    pub video_subsystem: VideoSubsystem,
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
    pub max_fps: u32,
    pub delta: Instant,
    keys: HashMap<Keycode, bool>,
    input_actions: HashMap<&'a str, InputAction>,
}

/// Initialization and setup related impl
impl GameContext<'_> {
    /// Sets up a window and game context.
    pub fn setup(title: &str, width: u32, height: u32) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window(title, width, height)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();

        Self::new(sdl_context, video_subsystem, canvas, event_pump)
    }

    /// Creates a GameContext
    pub fn new(
        sdl_context: Sdl,
        video_subsystem: VideoSubsystem,
        canvas: Canvas<Window>,
        event_pump: EventPump,
    ) -> Self {
        GameContext {
            sdl_context,
            video_subsystem,
            canvas,
            event_pump,
            max_fps: 60,
            delta: Instant::now(),
            keys: HashMap::new(),
            input_actions: HashMap::new(),
        }
    }
}

/// Gameloop related impl
impl GameContext<'_> {
    /// Starts the game loop, running the provided callable every frame.
    pub fn tick<F: FnMut(&mut GameContext)>(&mut self, mut f: F) {
        'running: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,

                    Event::KeyDown { keycode, .. } => {
                        if let Some(value) = keycode {
                            self.keys.insert(value, true);
                        }
                    }

                    Event::KeyUp { keycode, .. } => {
                        if let Some(value) = keycode {
                            self.keys.insert(value, false);
                        }
                    }

                    _ => (),
                }
            }

            f(self);

            self.delta = Instant::now();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / self.max_fps));
        }
    }
}

/// Keyboard related impl
impl<'a> GameContext<'a> {
    /// Special syntax sugar for registering multiple actions.
    /// # Examples
    /// ```rust
    /// c.register_actions([
    ///     ("up", [Keycode::W, Keycode::Up]),
    ///     ("left", [Keycode::A, Keycode::Left]),
    ///     ("down", [Keycode::S, Keycode::Down]),
    ///     ("right", [Keycode::D, Keycode::Right])
    /// ]);
    /// ```
    pub fn register_actions(
        &mut self,
        actions: impl IntoIterator<Item = (&'a str, impl Into<Vec<Keycode>>)>,
    ) {
        for (key, action) in actions {
            let _ = self.register_action(key, InputAction::new(action.into()));
        }
    }

    /// Unregister and return the action if it exists.
    pub fn unregister_action(&mut self, key: &str) -> Option<InputAction> {
        self.input_actions.remove(key)
    }

    /// Register the action. Return Err if an action is already registered using that key.
    pub fn register_action(
        &mut self,
        key: &'a str,
        action: InputAction,
    ) -> Result<(), (&'a str, InputAction)> {
        if self.input_actions.contains_key(key) {
            return Err((key, action));
        }

        match self.input_actions.insert(key, action) {
            None => Ok(()),
            Some(_) => unreachable!("Core error: Input actions shouldn't contain this key!"),
        }
    }

    /// Checks if the specific action is down
    pub fn is_action_down(&self, key: &'a str) -> bool {
        match self.input_actions.get(key) {
            Some(value) => value.keys.iter().any(|x| self.is_key_down(x)),
            None => false,
        }
    }

    /// Checks if the specific action is up
    pub fn is_action_up(&self, key: &'a str) -> bool {
        !self.is_action_down(key)
    }

    /// Checks if the specific key is down.
    pub fn is_key_down(&self, key: &Keycode) -> bool {
        match self.keys.get(key) {
            Some(value) => *value,
            None => false,
        }
    }

    /// Checks if the specific key is up.
    pub fn is_key_up(&self, key: &Keycode) -> bool {
        !self.is_key_down(key)
    }
}
