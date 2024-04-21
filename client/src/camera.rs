use std::f32;

use vek::{Mat4, Vec2, Vec3};

const NEAR_PLANE: f32 = 0.1;
const FAR_PLANE: f32 = 1000.0;

pub struct Matrices {
    pub proj: Mat4<f32>,
    pub view: Mat4<f32>,
}

pub struct Camera {
    pos: Vec3<f32>,
    rotation: Vec2<f32>,
    aspect: f32,
    matrices: Matrices,
    fov: f32,
}

impl Camera {
    pub fn new(aspect: f32) -> Self {
        Self {
            pos: Vec3::new(0.0, 0.0, -2.0),
            rotation: Vec2::new(-1.5, 0.0),
            aspect,
            fov: f32::consts::FRAC_PI_2,
            matrices: Matrices {
                proj: Mat4::perspective_lh_no(
                    f32::consts::FRAC_PI_2,
                    aspect,
                    NEAR_PLANE,
                    FAR_PLANE,
                ),
                view: Mat4::identity(),
            },
        }
    }

    pub fn set_aspect_ratio(&mut self, aspect: f32) {
        self.aspect = aspect;
        self.matrices.proj = Mat4::perspective_lh_no(self.fov, aspect, NEAR_PLANE, FAR_PLANE);
    }

    pub fn rotate_by(&mut self, dx: f32, dy: f32) {
        self.rotation.x += dx.to_radians();
        self.rotation.y += -dy.to_radians();
        self.rotation.y = self.rotation.y.clamp(
            -f32::consts::FRAC_PI_2 + 0.0001,
            f32::consts::FRAC_PI_2 - 0.0001,
        );
    }

    pub fn compute_matrices(&mut self) -> Matrices {
        self.matrices.view = Mat4::look_at_lh(self.pos, self.pos + self.forward(), Vec3::unit_y());
        Matrices {
            proj: self.matrices.proj,
            view: self.matrices.view,
        }
    }

    pub fn move_by(&mut self, dx: f32, dy: f32, dz: f32) {
        self.pos += dz * self.forward_xz() + -dx * self.right() + Vec3::unit_y() * dy;
    }

    pub fn right(&self) -> Vec3<f32> {
        self.forward().cross(Vec3::unit_y()).normalized()
    }

    pub fn forward(&self) -> Vec3<f32> {
        Vec3::new(
            f32::cos(self.rotation.x) * f32::cos(self.rotation.y),
            f32::sin(self.rotation.y),
            -f32::sin(self.rotation.x) * f32::cos(self.rotation.y),
        )
        .normalized()
    }

    pub fn forward_xz(&self) -> Vec3<f32> {
        Vec3::new(f32::cos(self.rotation.x), 0.0, -f32::sin(self.rotation.x)).normalized()
    }
}
