use ultraviolet::{Mat4, Vec3};
use winit::event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent};

pub struct CameraController {
    speed: f32,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
        }
    }

    pub fn process_events(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state,
                        virtual_keycode: Some(keycode),
                        ..
                    },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;
                match keycode {
                    VirtualKeyCode::W => {
                        self.is_forward_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::A => {
                        self.is_left_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::S => {
                        self.is_backward_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::D => {
                        self.is_right_pressed = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    pub fn update_camera(&self, camera: &mut Camera) {
        if self.is_forward_pressed {
            camera.eye.z -= self.speed;
        }
        if self.is_backward_pressed {
            camera.eye.z += self.speed;
        }
        if self.is_left_pressed {
            camera.eye.x -= self.speed;
        }
        if self.is_right_pressed {
            camera.eye.x += self.speed;
        }
    }
}

pub struct Camera {
    pub eye: Vec3,
    pub direction: Vec3,
    pub up: Vec3,
    pub aspect: f32,
    /// Radians
    pub fov_y: f32,
    pub z_near: f32,
    pub z_far: f32,
}

impl Camera {
    pub fn build_view_projection_matrix(&self) -> (Mat4, Mat4) {
        let view = Mat4::look_at(self.eye, self.eye + self.direction, self.up);
        //let ortho_proj =
        //    ultraviolet::projection::orthographic_wgpu_dx(-8., 8., -6., 6., 1.0, 1000.);
        //(view, ortho_proj)
        let perspective_proj = ultraviolet::projection::rh_yup::perspective_wgpu_dx(
            self.fov_y,
            self.aspect,
            self.z_near,
            self.z_far,
        );
        (view, perspective_proj)
    }
}
