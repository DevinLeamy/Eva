use nalgebra::{Matrix4, Unit, Vector3};

#[derive(Clone, Debug)]
pub struct Camera {
    position: Vector3<f32>,
    /// Field of view in degrees.
    fov: f32,
    target: Unit<Vector3<f32>>,
    up: Unit<Vector3<f32>>,
}

impl Camera {
    pub fn new(position: Vector3<f32>, fov: f32, target: Vector3<f32>, up: Vector3<f32>) -> Self {
        Self {
            position,
            fov,
            target: Unit::new_normalize(target),
            up: Unit::new_normalize(up),
        }
    }
}

impl Camera {
    pub fn set_position(&mut self, position: Vector3<f32>) {
        self.position = position;
    }

    pub fn translate(&mut self, translation: Vector3<f32>) {
        self.position += translation;
    }

    pub fn set_fov(&mut self, fov: f32) {
        self.fov = fov;
    }

    pub fn set_target(&mut self, target: Vector3<f32>) {
        self.target = Unit::new_normalize(target);
    }

    pub fn set_up(&mut self, up: Vector3<f32>) {
        self.up = Unit::new_normalize(up);
    }

    pub fn look_at(&mut self, position: Vector3<f32>) {
        self.target = Unit::new_normalize(position - self.position);
    }

    pub fn origin(&self) -> Vector3<f32> {
        self.position
    }

    pub fn fov(&self) -> f32 {
        self.fov
    }

    /// Matrix to convert from camera space - centered at (0, 0, 0) and looking
    /// down the -z axis) to world space.
    pub fn camera_to_world_mat(&self) -> Matrix4<f32> {
        let target = self.position + self.target.into_inner();
        let world_to_camera = Matrix4::look_at_lh(&self.position.into(), &target.into(), &self.up);

        world_to_camera.try_inverse().unwrap()
    }
}
