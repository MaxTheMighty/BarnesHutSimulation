use glam::Vec3;

#[derive(Debug,Copy,Clone)]
pub struct Body3{
    pub pos: Vec3,
    pub acceleration: Vec3,
    pub velocity: Vec3,
    pub mass: f64,
    pub force: Vec3
}


impl Body3{
    pub fn new() -> Body3{
        Self {
            pos: Vec3::new(0.0,0.0,0.0),
            acceleration: Vec3::new(0.0,0.0,0.0),
            velocity: Vec3::new(0.0,0.0,0.0),
            mass: 0.0,
            force: Vec3::new(0.0,0.0,0.0)
        }
    }

    pub fn from_mass_and_pos(mass: f64, pos: Vec3) -> Body3{
        Self {
            pos: pos,
            acceleration: Vec3::new(0.0,0.0,0.0),
            velocity: Vec3::new(0.0,0.0,0.0),
            mass: mass,
            force: Vec3::new(0.0,0.0,0.0)
        }
    }
}