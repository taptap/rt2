use na::Vector3 as Vec3;
use animation::SetPosition;

#[derive(Clone, Copy)]
pub struct Light {
    pub pos: Vec3<f64>,
    pub color: Vec3<f64>,
}

impl SetPosition for Light {
    fn set_position(&mut self, pos: Vec3<f64>) {
        self.pos = pos;
    }

    fn get_position(&self) -> Vec3<f64> {
        self.pos
    }
}
