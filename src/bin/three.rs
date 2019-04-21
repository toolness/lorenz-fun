use kiss3d::window::{Window, State};
use kiss3d::light::Light;
use kiss3d::scene::SceneNode;
use nalgebra::{Vector3, UnitQuaternion, Translation3};

struct AppState {
    lz: lorenz::Lorenz,
    rot: UnitQuaternion<f32>,
    cube: SceneNode
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

        cube.set_color(1.0, 0.0, 0.0);
        AppState { cube, rot, lz }
    }
}

impl State for AppState {
    fn step(&mut self, _window: &mut Window) {
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
    }
}

fn main() {
    let mut window = Window::new("lorenz-fun");
    let app = AppState::new(&mut window);

    window.set_light(Light::StickToCamera);

    window.render_loop(app);
}
