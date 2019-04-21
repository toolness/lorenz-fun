use kiss3d::window::{Window, State};
use kiss3d::event::{Action, WindowEvent, Key};
use kiss3d::light::Light;
use kiss3d::text::Font;
use kiss3d::scene::SceneNode;
use std::collections::vec_deque::VecDeque;
use nalgebra::{Vector3, UnitQuaternion, Translation3, Point3, Point2};

const POINT_TRAIL_LEN: usize = 500;
const POINT_TRAIL_INTENSITY_DECAY: f32 = 1.0 / POINT_TRAIL_LEN as f32;
const HELP_LINES: [&'static str; 4] = [
    "Press '1' or '2' to try different Lorenz configurations.",
    "Scroll the mousewheel to zoom.",
    "Left-click drag to rotate the camera, right-click drag to pan it.",
    "Press 'h' to toggle this help text."
];

enum LorenzConfig {
    One,
    Two
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
        for point in self.trail.iter() {
            let color = intensity * self.color;
            window.draw_point(point, &color);
            intensity -= POINT_TRAIL_INTENSITY_DECAY;
        }
    }

    fn remove(&mut self, window: &mut Window) {
        window.remove_node(&mut self.head);
    }
}

struct AppState {
    l3ds: Vec<Lorenz3d>,
    font: std::rc::Rc<Font>,
    show_help: bool
}

impl AppState {
    fn new() -> Self {
        AppState {
            l3ds: Vec::new(),
            font: Font::default(),
            show_help: true
        }
    }

    fn clear(&mut self, window: &mut Window) {
        for l3d in self.l3ds.iter_mut() {
            l3d.remove(window);
        }
        self.l3ds.clear();
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
}

impl State for AppState {
    fn step(&mut self, window: &mut Window) {
        for mut event in window.events().iter() {
            match event.value {
                WindowEvent::Key(button, Action::Press, _) => {
                    match button {
                        Key::Key1 => {
                            self.init_config(window, LorenzConfig::One);
                            event.inhibited = true;
                        },
                        Key::Key2 => {
                            self.init_config(window, LorenzConfig::Two);
                            event.inhibited = true;
                        },
                        Key::H => {
                            self.show_help = !self.show_help;
                        },
                        _ => {}
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

fn main() {
    let mut window = Window::new("lorenz-fun");
    let mut app = AppState::new();

    window.set_light(Light::StickToCamera);
    window.set_point_size(1.0);
    app.init_config(&mut window, LorenzConfig::One);

    window.render_loop(app);
}
