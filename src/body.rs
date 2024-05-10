use cgmath::Vector2;
#[derive(Debug)]
pub struct Body {
    pub pos: Vector2<f64>,
    pub acceleration: Vector2<f64>,
    pub velocity: Vector2<f64>,
    pub mass: f64,
}


impl Body{
    pub fn new() -> Body{
        Self {
            pos: Vector2::new(0.0,0.0),
            acceleration: Vector2::new(0.0,0.0),
            velocity: Vector2::new(0.0,0.0),
            mass: 0.0
        }
    }

    pub fn random() -> Body{
        let mut out = Self::with_mass_and_pos(1.0f64,Vector2::new((fastrand::f64()*50.0f64) + 500.0f64, (fastrand::f64()*50.0f64) + 500.0f64));
        // out.velocity = Vector2::new(fastrand::f64()*2.0f64 - 1.0f64, fastrand::f64()*2.0f64 - 1.0f64);
        return out;
    }

    pub fn with_mass(mass: f64) -> Body {
        Self {
            pos: Vector2::new(0.0,0.0),
            acceleration: Vector2::new(0.0,0.0),
            velocity: Vector2::new(0.0,0.0),
            mass,
        }
    }

    pub fn with_mass_and_pos(mass: f64, pos: Vector2<f64>) -> Body {
        Self {
            pos,
            acceleration: Vector2::new(0.0,0.0),
            velocity: Vector2::new(0.0,0.0),
            mass,
        }
    }



}
