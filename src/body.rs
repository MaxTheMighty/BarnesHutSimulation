use cgmath::Vector2;
#[derive(Debug)]
pub struct Body {
    pub pos: Vector2<f32>,
    pub acceleration: Vector2<f32>,
    pub velocity: Vector2<f32>,
    pub mass: f32,
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

    pub fn with_mass(mass: f32) -> Body {
        Self {
            pos: Vector2::new(0.0,0.0),
            acceleration: Vector2::new(0.0,0.0),
            velocity: Vector2::new(0.0,0.0),
            mass: mass,
        }
    }

    pub fn with_mass_and_pos(mass: f32, pos: Vector2<f32>) -> Body {
        Self {
            pos: pos,
            acceleration: Vector2::new(0.0,0.0),
            velocity: Vector2::new(0.0,0.0),
            mass: mass,
        }
    }



}
