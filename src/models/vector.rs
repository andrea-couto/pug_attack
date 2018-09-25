pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64) -> Self {
        Vector { x: x, y: y }
    }

    pub fn reset(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
    }

    pub fn dist(&self, other: &Vector) -> f64 {
        let xdiff = self.x - other.x;
        let ydiff = self.y - other.y;
        (xdiff.powi(2) + ydiff.powi(2)).sqrt()
    }
}
