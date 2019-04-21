use kiss3d::window::{Window, State};
use kiss3d::light::Light;
use kiss3d::scene::SceneNode;
use std::collections::vec_deque::VecDeque;
use nalgebra::{Vector3, UnitQuaternion, Translation3, Point3};

const POINT_TRAIL_LEN: usize = 500;
const POINT_TRAIL_INTENSITY_DECAY: f32 = 1.0 / POINT_TRAIL_LEN as f32;

struct Lorenz3d {
    lz: lorenz::Lorenz,
    head_rot: UnitQuaternion<f32>,
    head: SceneNode,
    trail: VecDeque<Point3<f32>>
}

impl Lorenz3d {
    fn new(window: &mut Window) -> Self {
        let head = window.add_cube(0.15, 0.15, 0.15);
        let head_rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);
        let lz = lorenz::Lorenz { .. Default::default() };
        let trail = VecDeque::with_capacity(POINT_TRAIL_LEN + 1);

        Lorenz3d { lz, head_rot, head, trail }
    }

    fn with_pos(mut self, x: f64, y: f64, z: f64) -> Self {
        self.lz.x = x;
        self.lz.y = y;
        self.lz.z = z;
        self
    }

    fn with_color(mut self, r: f32, g: f32, b: f32) -> Self {
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
        let mut c = 1.0;
        for point in self.trail.iter() {
            window.draw_point(point, &Point3::new(c, c, c));
            c -= POINT_TRAIL_INTENSITY_DECAY;
        }
    }
}

struct AppState {
    l3d: Lorenz3d
}

impl AppState {
    fn new(window: &mut Window) -> Self {
        let l3d = Lorenz3d::new(window)
          .with_pos(0.1, 0.1, 0.1)
          .with_color(1.0, 0.0, 0.0);
        AppState { l3d }
    }
}

impl State for AppState {
    fn step(&mut self, window: &mut Window) {
        self.l3d.step(window);
    }
}

fn main() {
    let mut window = Window::new("lorenz-fun");
    let app = AppState::new(&mut window);

    window.set_light(Light::StickToCamera);
    window.set_point_size(1.0);

    window.render_loop(app);
}
