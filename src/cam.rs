
//! A 3D camera.

use vecmath::{
    Vector3,
    Matrix3x4,
    vec3_normalized_sub,
    base4x3_mat,
    mat3x4_inv,
    vec3_cross,
};

pub struct Camera {
    pub position: Vector3,
    pub target: Vector3,
    pub right: Vector3,
    pub up: Vector3,
}

pub struct CameraSettings {
    pub fov_rad: f64,
    pub near_clip: f64,
    pub far_clip: f64,
    pub aspect_ratio: f64,
}

impl Camera {
    /// Computes the direction forward.
    ///
    /// Returns the normalized difference between target and position.
    pub fn forward(&self) -> Vector3 {
        vec3_normalized_sub(self.target, self.position)
    }

    /// Computes an orthogonal matrix for the camera.
    ///
    /// This matrix can be used to transform coordinates to the screen.
    pub fn orthogonal(&self) -> Matrix3x4 {
        mat3x4_inv(base4x3_mat([
            self.right,
            self.up,
            self.forward(),
            self.position
        ]))
    }

    pub fn update_right(&mut self) {
        self.right = vec3_cross(self.forward(), self.up);
    }
}

