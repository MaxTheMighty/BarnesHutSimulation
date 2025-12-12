use std::ops::{AddAssign, SubAssign};

use cgmath::ElementWise;
use winit::keyboard::KeyCode;

#[derive(Debug)]
pub struct Camera {
    pub eye: cgmath::Point3<f32>,
    pub target: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32
}


impl Camera {
    // Build a view projection matrix out of values defined in self
    // The view matrix translates the world to be in relation to the camera. 
    //     This is why the view matrix is built from the values of the camera. It's dependent on the camera
    // The projection matrix warps everything to give the scene a depth effect. Hence, projection
    fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);

        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);
    
        // Combine the view and project matricies and convert the coordinates from OPENGL to WGPU systems
        return OPENGL_TO_WGPU_MATRIX * proj * view;

    }
}

#[repr(C)]
#[derive(Debug,Copy,Clone,bytemuck::Pod,bytemuck::Zeroable)]
pub struct CameraUniform {
    // We can't use a cgmath matrix with bytemuck so we will convert it to a raw 2D f32 array
    pub view_proj: [[f32;4]; 4]
}

impl CameraUniform {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera){
        self.view_proj = camera.build_view_projection_matrix().into();
    }
}


#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::from_cols(
    cgmath::Vector4::new(1.0, 0.0, 0.0, 0.0),
    cgmath::Vector4::new(0.0, 1.0, 0.0, 0.0),
    cgmath::Vector4::new(0.0, 0.0, 0.5, 0.0),
    cgmath::Vector4::new(0.0, 0.0, 0.5, 1.0),
);

#[derive(Debug,Clone)]
pub struct CameraController {
    pub speed: f32,
}

impl CameraController {
    pub fn new() -> CameraController {
        CameraController { speed: 1.0f32 }
    }

    pub fn handle_key_code(&mut self, camera: &mut Camera, keycode: KeyCode) {
        match keycode {
            KeyCode::ArrowUp => {
                camera.eye.sub_assign_element_wise(self.speed);
            }
            KeyCode::ArrowDown => {
                camera.eye.add_assign_element_wise(self.speed);
            }
            KeyCode::ArrowRight => {
                camera.target.x.add_assign(self.speed);
            }
            KeyCode::ArrowLeft => {
                camera.target.x.sub_assign(self.speed);
            }
            _ => {}
        }
    }
}