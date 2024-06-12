use cgmath::Vector2;

#[derive(Debug,Copy,Clone)]
pub struct Body {
    pub pos: Vector2<f64>,
    pub acceleration: Vector2<f64>,
    pub velocity: Vector2<f64>,
    pub mass: f64,
    pub force: Vector2<f64>
}


impl Body{
    pub fn new() -> Body{
        Self {
            pos: Vector2::new(0.0,0.0,),
            acceleration: Vector2::new(0.0,0.0),
            velocity: Vector2::new(0.0,0.0),
            mass: 0.0,
            force: Vector2::new(0.0,0.0)
        }
    }

    pub fn random(offset: f64, area: f64) -> Body{
        let out = Self::with_mass_and_pos(10.0f64,Vector2::new((fastrand::f64()*area) + offset, (fastrand::f64()*area) + offset));
        // out.velocity = Vector2::new(fastrand::f64()*2.0f64 - 1.0f64, fastrand::f64()*2.0f64 - 1.0f64);
        return out;
    }

    pub fn with_mass(mass: f64) -> Body {
        Self {
            pos: Vector2::new(0.0,0.0),
            acceleration: Vector2::new(0.0,0.0),
            velocity: Vector2::new(0.0,0.0),
            mass,
            force: Vector2::new(0.0,0.0)
        }
    }

    pub fn with_mass_and_pos(mass: f64, pos: Vector2<f64>) -> Body {
        Self {
            pos,
            acceleration: Vector2::new(0.0,0.0),
            velocity: Vector2::new(0.0,0.0),
            mass,
            force: Vector2::new(0.0,0.0)
        }
    }

    pub fn with_pos(pos: Vector2<f64>) -> Body {
        Self {
            pos,
            acceleration: Vector2::new(0.0,0.0),
            velocity: Vector2::new(0.0,0.0),
            mass: 1.0,
            force: Vector2::new(0.0,0.0)
        }
    }






}
