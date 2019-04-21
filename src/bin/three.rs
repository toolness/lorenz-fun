use kiss3d::window::{Window, State};
use kiss3d::light::Light;
use kiss3d::scene::SceneNode;
use std::collections::vec_deque::VecDeque;
use nalgebra::{Vector3, UnitQuaternion, Translation3, Point3};

const POINT_TRAIL_LEN: usize = 500;
const POINT_TRAIL_INTENSITY_DECAY: f32 = 1.0 / POINT_TRAIL_LEN as f32;

struct AppState {
    lz: lorenz::Lorenz,
    rot: UnitQuaternion<f32>,
    cube: SceneNode,
    points: VecDeque<Point3<f32>>
}

impl AppState {
    fn new(window: &mut Window) -> Self {
        let mut cube = window.add_cube(0.15, 0.15, 0.15);
        let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);
        let lz = lorenz::Lorenz {
            x: 0.1,
            y: 0.1,
            z: 0.1,
            .. Default::default()
        };
        let points = VecDeque::with_capacity(POINT_TRAIL_LEN + 1);

        cube.set_color(1.0, 0.0, 0.0);
        AppState { cube, rot, lz, points }
    }
}

impl State for AppState {
    fn step(&mut self, window: &mut Window) {
        self.lz.update(0.01);
        let scale = 0.1;
        let vector = Vector3::new(
            self.lz.x as f32 * scale,
            self.lz.y as f32 * scale,
            self.lz.z as f32 * scale
        );
        let t = Translation3::from(vector);
        self.cube.set_local_translation(t);
        self.cube.prepend_to_local_rotation(&self.rot);
        self.points.push_front(Point3::from(vector));
        self.points.truncate(POINT_TRAIL_LEN);
        let mut c = 1.0;
        for point in self.points.iter() {
            window.draw_point(point, &Point3::new(c, c, c));
            c -= POINT_TRAIL_INTENSITY_DECAY;
        }
    }
}

fn main() {
    let mut window = Window::new("lorenz-fun");
    let app = AppState::new(&mut window);

    window.set_light(Light::StickToCamera);
    window.set_point_size(1.0);

    window.render_loop(app);
}
