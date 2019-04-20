pub struct Lorenz {
    pub sigma: f64,
    pub beta: f64,
    pub rho: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Lorenz {
    fn dxdt(&self) -> f64 {
        self.sigma * (self.y - self.x)
    }

    fn dydt(&self) -> f64 {
        self.x * (self.rho - self.z) - self.y
    }

    fn dzdt(&self) -> f64 {
        self.x * self.y - self.beta * self.z
    }

    pub fn update(&mut self, dt: f64) {
        let dx = self.dxdt() * dt;
        let dy = self.dydt() * dt;
        let dz = self.dzdt() * dt;

        self.x += dx;
        self.y += dy;
        self.z += dz;
    }
}

impl Default for Lorenz {
    fn default() -> Self {
        Lorenz {
            sigma: 10.0,
            beta: 8.0 / 3.0,
            rho: 28.0,
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
    }
}
