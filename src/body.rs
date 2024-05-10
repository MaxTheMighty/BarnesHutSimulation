use cgmath::Vector3;

#[derive(Debug)]
pub struct Body {
    pub pos: Vector3<f64>,
    pub acceleration: Vector3<f64>,
    pub velocity: Vector3<f64>,
    pub mass: f64,
}


impl Body{
    pub fn new() -> Body{
        Self {
            pos: Vector3::new(0.0,0.0,0.0),
            acceleration: Vector3::new(0.0,0.0,0.0),
            velocity: Vector3::new(0.0,0.0,0.0),
            mass: 0.0
        }
    }

    pub fn random(offset: f64, area: f64) -> Body{
        let mut out = Self::with_mass_and_pos(10.0f64,Vector3::new((fastrand::f64()*area) + offset, (fastrand::f64()*area) + offset,(fastrand::f64()*area) + offset));
        // out.velocity = Vector3::new(fastrand::f64()*2.0f64 - 1.0f64, fastrand::f64()*2.0f64 - 1.0f64);
        return out;
    }

    pub fn with_mass(mass: f64) -> Body {
        Self {
            pos: Vector3::new(0.0,0.0,0.0),
            acceleration: Vector3::new(0.0,0.0,0.0),
            velocity: Vector3::new(0.0,0.0,0.0),
            mass,
        }
    }

    pub fn with_mass_and_pos(mass: f64, pos: Vector3<f64>) -> Body {
        Self {
            pos,
            acceleration: Vector3::new(0.0,0.0,0.0),
            velocity: Vector3::new(0.0,0.0,0.0),
            mass,
        }
    }






}
