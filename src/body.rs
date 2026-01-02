use cgmath::Vector2;

pub type Float = f32;
pub const BYTES_PER_BODY: usize = size_of::<Float>() * (2 + 2 + 2 + 1 + 2);
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct Body {
    pub pos: Vector2<Float>,
    pub acceleration: Vector2<Float>,
    pub velocity: Vector2<Float>,
    pub mass: Float,
    pub force: Vector2<Float>,
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct BodyCollection(pub Vec<Body>);

impl AsRef<Vec<Body>> for BodyCollection {
    fn as_ref(&self) -> &Vec<Body> {
        return &self.0;
    }
}

impl AsMut<Vec<Body>> for BodyCollection {
    fn as_mut(&mut self) -> &mut Vec<Body> {
        return &mut self.0;
    }
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

impl From<&Body> for Vec<u8> {
    fn from(value: &Body) -> Self {
        let mut bytes = Vec::new();
        // Pos
        bytes.extend_from_slice(&value.pos.x.to_le_bytes());
        bytes.extend_from_slice(&value.pos.y.to_le_bytes());

        // Acceleration
        bytes.extend_from_slice(&value.acceleration.x.to_le_bytes());
        bytes.extend_from_slice(&value.acceleration.y.to_le_bytes());

        // Velocity
        bytes.extend_from_slice(&value.velocity.x.to_le_bytes());
        bytes.extend_from_slice(&value.velocity.y.to_le_bytes());

        // Mass
        bytes.extend_from_slice(&value.mass.to_le_bytes());

        // Force
        bytes.extend_from_slice(&value.force.x.to_le_bytes());
        bytes.extend_from_slice(&value.force.y.to_le_bytes());

        return bytes;
    }
}

// Something tells me this function is HOT
impl From<&[u8]> for Body {
    fn from(value: &[u8]) -> Self {
        // since everything is f32, split into 4 byte parts
        let mut floats: Vec<f32> = Vec::with_capacity(2 + 2 + 2 + 1 + 2);
        for chunk in value.chunks_exact(4) {
            let f32_slice: [u8; 4] = chunk.try_into().expect("Chunk wrong size");
            let f32: f32 = f32::from_le_bytes(f32_slice);
            floats.push(f32);
        }
        return Self {
            pos: Vector2 {
                x: *floats.get(0).unwrap(),
                y: *floats.get(1).unwrap(),
            },
            acceleration: Vector2 {
                x: *floats.get(2).unwrap(),
                y: *floats.get(3).unwrap(),
            },
            velocity: Vector2 {
                x: *floats.get(4).unwrap(),
                y: *floats.get(5).unwrap(),
            },
            mass: *floats.get(6).unwrap(),
            force: Vector2 {
                x: *floats.get(7).unwrap(),
                y: *floats.get(8).unwrap(),
            },
        };
    }
}

impl From<&[u8]> for BodyCollection {
    fn from(value: &[u8]) -> Self {
        if value.len() % BYTES_PER_BODY != 0 {
            panic!("Not enough bytes in value");
        }

        let mut bodies: Vec<Body> = Vec::with_capacity((value.len() / BYTES_PER_BODY) as usize);
        for chunk in value.chunks_exact(BYTES_PER_BODY) {
            bodies.push(Body::from(chunk));
        }

        return BodyCollection { 0: bodies };
    }
}
#[cfg(test)]
mod tests {
    use cgmath::Vector2;

    use crate::body::{Body, BodyCollection};

    #[test]
    fn to_vec_u8() {
        let body: Body = Body::new();
        let bytes: Vec<u8> = (&body).into();
        assert!(bytes.iter().all(|b| *b == 0u8));
    }

    #[test]
    fn to_vec_u8_two() {
        let body: Body = Body::with_mass(1.0f32);
        let bytes: Vec<u8> = (&body).into();
        assert!(bytes.iter().any(|b| *b != 0u8));
    }

    #[test]
    fn from_vec_u8() {
        let body: Body = Body::with_pos(Vector2::new(12.34f32, 45.67f32));
        let bytes: Vec<u8> = (&body).into();
        let new_body: Body = (bytes[..]).into();
        assert_eq!(body, new_body);
    }

    #[test]
    fn from_vec_u8_fuzz() {
        for _ in 0..1_000_000 {
            let rand_body: Body = Body::random(0.0, 1000.0);
            let bytes: Vec<u8> = (&rand_body).into();
            let new_body: Body = (&bytes[..]).into();
            assert_eq!(rand_body, new_body);
        }
    }

    #[test]
    fn from_vec_u8_collection() {
        let body: Body = Body::with_pos(Vector2::new(12.34f32, 45.67f32));
        let mut bytes: Vec<u8> = (&body).into();
        let body2: Body = Body::with_pos(Vector2::new(0.0001f32, 98.7654321f32));
        let mut bytes2: Vec<u8> = (&body2).into();
        bytes.append(&mut bytes2);
        let bodies: BodyCollection = (bytes[..]).into();
        assert_eq!(bodies.0.first().unwrap(), &body);
        assert_eq!(bodies.0.last().unwrap(), &body2);
    }
}
