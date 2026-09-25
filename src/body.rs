use bytemuck::{Pod, Zeroable};
use glam::f32::Vec2;

pub type Float = f32;
pub const BYTES_PER_BODY: usize = std::mem::size_of::<Body>();
#[derive(Debug, Copy, Clone, PartialEq, Pod, Zeroable)]
#[repr(C)]
pub struct Body {
    pub pos: Vec2,          // 8 bytes
    pub acceleration: Vec2, // 8 bytes
    pub velocity: Vec2,     // 8 bytes
    pub force: Vec2,        // 8 bytes
    pub mass: Float,        // 4 bytes
    pub _padding: Float,    // 4 bytes
}

impl Body {
    pub fn new() -> Body {
        Self {
            pos: Vec2::new(0.0, 0.0),
            acceleration: Vec2::new(0.0, 0.0),
            velocity: Vec2::new(0.0, 0.0),
            mass: 0.0,
            force: Vec2::new(0.0, 0.0),
            _padding: 0.0,
        }
    }

    pub fn random(offset: Float, area: Float) -> Body {
        let out = Self::with_mass_and_pos(
            10.0,
            Vec2::new(
                ((fastrand::f32() * area) + offset),
                ((fastrand::f32() * area) + offset),
            ),
        );
        // out.velocity = Vec2::new(fastrand::Float()*2.0Float - 1.0Float, fastrand::Float()*2.0Float - 1.0Float);
        return out;
    }

    pub fn with_mass(mass: Float) -> Body {
        Self {
            pos: Vec2::new(0.0, 0.0),
            acceleration: Vec2::new(0.0, 0.0),
            velocity: Vec2::new(0.0, 0.0),
            mass,
            force: Vec2::new(0.0, 0.0),
            _padding: 0.0,
        }
    }

    pub fn with_mass_and_pos(mass: Float, pos: Vec2) -> Body {
        Self {
            pos,
            acceleration: Vec2::new(0.0, 0.0),
            velocity: Vec2::new(0.0, 0.0),
            mass,
            force: Vec2::new(0.0, 0.0),
            _padding: 0.0,
        }
    }

    pub fn with_pos(pos: Vec2) -> Body {
        Self {
            pos,
            acceleration: Vec2::new(0.0, 0.0),
            velocity: Vec2::new(0.0, 0.0),
            mass: 1.0,
            force: Vec2::new(0.0, 0.0),
            _padding: 0.0,
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

// impl From<&Body> for Vec<u8> {
//     fn from(value: &Body) -> Self {
//         let mut bytes = Vec::new();
//         // Pos
//         bytes.extend_from_slice(&value.pos.x.to_le_bytes());
//         bytes.extend_from_slice(&value.pos.y.to_le_bytes());

//         // Acceleration
//         bytes.extend_from_slice(&value.acceleration.x.to_le_bytes());
//         bytes.extend_from_slice(&value.acceleration.y.to_le_bytes());

//         // Velocity
//         bytes.extend_from_slice(&value.velocity.x.to_le_bytes());
//         bytes.extend_from_slice(&value.velocity.y.to_le_bytes());

//         // Mass
//         bytes.extend_from_slice(&value.mass.to_le_bytes());

//         // Force
//         bytes.extend_from_slice(&value.force.x.to_le_bytes());
//         bytes.extend_from_slice(&value.force.y.to_le_bytes());

//         return bytes;
//     }
// }

// Something tells me this function is HOT
// impl From<&[u8]> for Body {
//     fn from(value: &[u8]) -> Self {
//         // since everything is f32, split into 4 byte parts
//         let mut floats: Vec<f32> = Vec::with_capacity(2 + 2 + 2 + 1 + 2);
//         for chunk in value.chunks_exact(4) {
//             let f32_slice: [u8; 4] = chunk.try_into().expect("Chunk wrong size");
//             let f32: f32 = f32::from_le_bytes(f32_slice);
//             floats.push(f32);
//         }
//         return Self {
//             pos: Vec2 {
//                 x: *floats.get(0).unwrap(),
//                 y: *floats.get(1).unwrap(),
//             },
//             acceleration: Vec2 {
//                 x: *floats.get(2).unwrap(),
//                 y: *floats.get(3).unwrap(),
//             },
//             velocity: Vec2 {
//                 x: *floats.get(4).unwrap(),
//                 y: *floats.get(5).unwrap(),
//             },
//             mass: *floats.get(6).unwrap(),
//             force: Vec2 {
//                 x: *floats.get(7).unwrap(),
//                 y: *floats.get(8).unwrap(),
//             },
//         };
//     }
// }

// impl From<&[u8]> for BodyCollection {
//     fn from(value: &[u8]) -> Self {
//         if value.len() % BYTES_PER_BODY != 0 {
//             panic!("Not enough bytes in value");
//         }

//         let mut bodies: Vec<Body> = Vec::with_capacity((value.len() / BYTES_PER_BODY) as usize);
//         for chunk in value.chunks_exact(BYTES_PER_BODY) {
//             bodies.push(Body::from(chunk));
//         }

//         return BodyCollection { 0: bodies };
//     }
// }
#[cfg(test)]
mod tests {

    use crate::body::{Body, BYTES_PER_BODY};
    use glam::f32::Vec2;

    #[test]
    fn to_vec_u8() {
        let body: Body = Body::new();
        let bytes: &[u8] = bytemuck::bytes_of(&body);
        assert!(bytes.iter().all(|b| *b == 0u8));
    }

    #[test]
    fn to_vec_u8_two() {
        let body: Body = Body::with_mass(1.0f32);
        let bytes: &[u8] = bytemuck::bytes_of(&body);
        assert!(bytes.iter().any(|b| *b != 0u8));
    }

    #[test]
    fn from_vec_u8() {
        let body: Body = Body::with_pos(Vec2::new(12.34f32, 45.67f32));
        let bytes: &[u8] = bytemuck::bytes_of(&body);
        let new_body: &Body = bytemuck::from_bytes(bytes);
        assert_eq!(&body, new_body);
    }

    #[test]
    fn from_vec_u8_fuzz() {
        for _ in 0..1_000_000 {
            let rand_body: Body = Body::random(0.0, 1000.0);
            let bytes: &[u8] = bytemuck::bytes_of(&rand_body);
            let new_body: &Body = bytemuck::from_bytes(bytes);
            assert_eq!(&rand_body, new_body);
        }
    }

    #[test]
    fn from_vec_u8_collection() {
        let body: Body = Body::with_pos(Vec2::new(12.34f32, 45.67f32));
        let bytes: &[u8] = bytemuck::bytes_of(&body);
        let body2: Body = Body::with_pos(Vec2::new(0.0001f32, 98.7654321f32));
        let bytes2: &[u8] = bytemuck::bytes_of(&body2);
        let mut bytes_combined: Vec<u8> = bytes.to_vec();
        bytes_combined.append(&mut bytes2.to_vec());
        let bodies: &[Body] = bytemuck::cast_slice(bytes_combined.as_slice());
        assert_eq!(bodies[0], body);
        assert_eq!(bodies[1], body2);
    }

    #[test]
    fn is_wgsl_aligned() {
        dbg!(BYTES_PER_BODY);
        assert!(BYTES_PER_BODY % 8 == 0);
    }
}
