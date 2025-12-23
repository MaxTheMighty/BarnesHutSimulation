use bytemuck::{bytes_of, NoUninit, Pod, Zeroable};
use cgmath::Vector2;

pub type Float = f32;
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Body {
    pub pos: Vector2<Float>,
    pub acceleration: Vector2<Float>,
    pub velocity: Vector2<Float>,
    pub mass: Float,
    pub force: Vector2<Float>,
}

impl Body {
    pub fn new() -> Body {
        Self {
            pos: Vector2::new(0.0, 0.0),
            acceleration: Vector2::new(0.0, 0.0),
            velocity: Vector2::new(0.0, 0.0),
            mass: 0.0,
            force: Vector2::new(0.0, 0.0),
        }
    }

    pub fn random(offset: Float, area: Float) -> Body {
        let out = Self::with_mass_and_pos(
            10.0,
            Vector2::new(
                ((fastrand::f32() * area) + offset),
                ((fastrand::f32() * area) + offset),
            ),
        );
        // out.velocity = Vector2::new(fastrand::Float()*2.0Float - 1.0Float, fastrand::Float()*2.0Float - 1.0Float);
        return out;
    }

    pub fn with_mass(mass: Float) -> Body {
        Self {
            pos: Vector2::new(0.0, 0.0),
            acceleration: Vector2::new(0.0, 0.0),
            velocity: Vector2::new(0.0, 0.0),
            mass,
            force: Vector2::new(0.0, 0.0),
        }
    }

    pub fn with_mass_and_pos(mass: Float, pos: Vector2<Float>) -> Body {
        Self {
            pos,
            acceleration: Vector2::new(0.0, 0.0),
            velocity: Vector2::new(0.0, 0.0),
            mass,
            force: Vector2::new(0.0, 0.0),
        }
    }

    pub fn with_pos(pos: Vector2<Float>) -> Body {
        Self {
            pos,
            acceleration: Vector2::new(0.0, 0.0),
            velocity: Vector2::new(0.0, 0.0),
            mass: 1.0,
            force: Vector2::new(0.0, 0.0),
        }
    }
}

impl std::fmt::Display for Body {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Pos: ({:0>20} {:0>20}) Force: ({:0>20} {:0>20})",
            self.pos.x, self.pos.y, self.force.x, self.force.y
        )
    }
}
