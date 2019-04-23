#[cfg(target_arch = "wasm32")]
#[macro_use]
extern crate stdweb;

use kiss3d::window::{Window, State};
use kiss3d::event::{Action, WindowEvent, Key};
use kiss3d::light::Light;
use kiss3d::text::Font;
use kiss3d::scene::SceneNode;
use rand::prelude::*;
use std::collections::vec_deque::VecDeque;
use nalgebra::{Vector3, UnitQuaternion, Translation3, Point3, Point2};

const POINT_TRAIL_LEN: usize = 1000;
const POINT_TRAIL_INTENSITY_DECAY: f32 = 1.0 / POINT_TRAIL_LEN as f32;
const HELP_LINES: [&'static str; 6] = [
    "Press '1' for Lorenz config 1.",
    "Press '2' for Lorenz config 2.",
    "Scroll the mousewheel to zoom.",
    "Left-click drag to rotate.",
    "Right-click drag to pan.",
    "Press 'h' to toggle this help text."
];

#[derive(Clone, Copy)]
enum LorenzConfig {
    One,
    Two
}

#[derive(Clone, Copy)]
enum AppAction {
    InitConfig(LorenzConfig),
    ToggleHelp,
    AddObj,
    RemoveObj
}

impl AppAction {
    fn from_key(key: Key) -> Option<Self> {
        match key {
            Key::Key1 => Some(AppAction::InitConfig(LorenzConfig::One)),
            Key::Key2 => Some(AppAction::InitConfig(LorenzConfig::Two)),
            Key::H => Some(AppAction::ToggleHelp),
            Key::Equals => Some(AppAction::AddObj),
            Key::Minus => Some(AppAction::RemoveObj),
            _ => None
        }
    }
}

struct Lorenz3d {
    lz: lorenz::Lorenz,
    color: Point3<f32>,
    head_rot: UnitQuaternion<f32>,
    head: SceneNode,
    trail: VecDeque<Point3<f32>>
}

impl Lorenz3d {
    fn new(window: &mut Window) -> Self {
        let color = Point3::new(1.0, 1.0, 1.0);
        let head = window.add_cube(0.15, 0.15, 0.15);
        let head_rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);
        let lz = lorenz::Lorenz { .. Default::default() };
        let trail = VecDeque::with_capacity(POINT_TRAIL_LEN + 1);

        Lorenz3d { lz, color, head_rot, head, trail }
    }

    fn with_pos(mut self, x: f64, y: f64, z: f64) -> Self {
        self.lz.x = x;
        self.lz.y = y;
        self.lz.z = z;
        self
    }

    fn with_color(mut self, r: f32, g: f32, b: f32) -> Self {
        self.color = Point3::new(r, g, b);
        self.head.set_color(r, g, b);
        self
    }

    fn step(&mut self, window: &mut Window) {
        self.lz.update(0.01);
        let scale = 0.1;
        let vector = Vector3::new(
            self.lz.x as f32 * scale,
            self.lz.y as f32 * scale,
            self.lz.z as f32 * scale
        );
        let t = Translation3::from(vector);
        self.head.set_local_translation(t);
        self.head.prepend_to_local_rotation(&self.head_rot);
        self.trail.push_front(Point3::from(vector));
        self.trail.truncate(POINT_TRAIL_LEN);
        let mut intensity = 1.0;
        let mut opt_last_point = None;
        for point in self.trail.iter() {
            if let Some(last_point) = opt_last_point {
                let color = intensity * self.color;
                window.draw_line(last_point, point, &color);
                intensity -= POINT_TRAIL_INTENSITY_DECAY;
            }
            opt_last_point = Some(point);
        }
    }

    fn remove(&mut self, window: &mut Window) {
        window.remove_node(&mut self.head);
    }
}

struct AppState {
    l3ds: Vec<Lorenz3d>,
    font: std::rc::Rc<Font>,
    show_help: bool,
    rng: ThreadRng
}

impl AppState {
    fn new() -> Self {
        AppState {
            l3ds: Vec::new(),
            font: Font::default(),
            show_help: true,
            rng: rand::thread_rng()
        }
    }

    fn clear(&mut self, window: &mut Window) {
        while self.l3ds.len() > 0 {
            self.remove_l3d(window);
        }
    }

    fn init_config(&mut self, window: &mut Window, config: LorenzConfig) {
        self.clear(window);
        match config {
            LorenzConfig::One => {
                self.l3ds.push(Lorenz3d::new(window)
                    .with_pos(0.1, 0.1, 0.1)
                    .with_color(1.0, 1.0, 1.0));
            },
            LorenzConfig::Two => {
                self.l3ds.push(Lorenz3d::new(window)
                    .with_pos(0.1, 0.1, 0.1)
                    .with_color(1.0, 0.0, 0.0));
                self.l3ds.push(Lorenz3d::new(window)
                    .with_pos(0.1000001, 0.1, 0.1)
                    .with_color(1.0, 0.0, 1.0));
            }
        }
    }

    fn add_random_l3d(&mut self, window: &mut Window) {
        let rng = &mut self.rng;
        let scale = 20.0;
        let ofs = -10.0;
        let l3d = Lorenz3d::new(window)
            .with_pos(
                ofs + rng.gen::<f64>() * scale,
                ofs + rng.gen::<f64>() * scale,
                ofs + rng.gen::<f64>() * scale
            )
            .with_color(rng.gen(), rng.gen(), rng.gen());
        self.l3ds.push(l3d);
    }

    fn remove_l3d(&mut self, window: &mut Window) {
        if let Some(mut l3d) = self.l3ds.pop() {
            l3d.remove(window);
        }
    }

    fn draw_help(&self, window: &mut Window) {
        let mut y = 8.0;
        for line in HELP_LINES.iter() {
            window.draw_text(
                line,
                &Point2::new(8.0, y),
                36.0,
                &self.font,
                &Point3::new(1.0, 1.0, 1.0)
            );
            y += 40.0;
        }
    }

    fn execute_action(&mut self, action: AppAction, window: &mut Window) {
        match action {
            AppAction::InitConfig(cfg) => self.init_config(window, cfg),
            AppAction::ToggleHelp => self.show_help = !self.show_help,
            AppAction::AddObj => self.add_random_l3d(window),
            AppAction::RemoveObj => self.remove_l3d(window)
        }
    }
}

impl State for AppState {
    fn step(&mut self, window: &mut Window) {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(config) = check_config_to_init() {
                self.init_config(window, config);
            }
        }

        for mut event in window.events().iter() {
            match event.value {
                WindowEvent::Key(button, Action::Press, _) => {
                    if let Some(action) = AppAction::from_key(button) {
                        event.inhibited = true;
                        self.execute_action(action, window);
                    }
                },
                _ => {}
            }
        }
        for l3d in self.l3ds.iter_mut() {
            l3d.step(window);
        }
        if self.show_help {
            self.draw_help(window);
        }
    }
}

#[cfg(target_arch = "wasm32")]
static mut CONFIG_TO_INIT: Option<LorenzConfig> = None;

#[cfg(target_arch = "wasm32")]
fn check_config_to_init() -> Option<LorenzConfig> {
    let result;
    unsafe {
        result = CONFIG_TO_INIT;
        CONFIG_TO_INIT = None;
    }
    result
}

#[cfg(target_arch = "wasm32")]
#[js_export]
fn init_config(config: i32) -> bool {
    match config {
        1 => {
            unsafe { CONFIG_TO_INIT = Some(LorenzConfig::One); }
            true
        },
        2 => {
            unsafe { CONFIG_TO_INIT = Some(LorenzConfig::Two); }
            true
        },
        _ => {
            false
        }
    }
}

fn main() {
    let mut window = Window::new("lorenz-fun");
    let mut app = AppState::new();

    window.set_light(Light::StickToCamera);
    app.init_config(&mut window, LorenzConfig::One);

    window.render_loop(app);
}
