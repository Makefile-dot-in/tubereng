#![warn(clippy::pedantic)]

use std::collections::HashMap;

use tubereng_math::{
    matrix::{Identity, Matrix4f},
    quaternion::Quaternion,
    vector::Vector3f,
};

pub struct DeltaTime(pub f32);

#[derive(Debug, Clone)]
pub struct Transform {
    pub translation: Vector3f,
    pub scale: Vector3f,
    pub rotation: Quaternion,
}

impl Transform {
    #[must_use]
    pub fn as_matrix4(&self) -> Matrix4f {
        Matrix4f::new_scale(&self.scale)
            * Matrix4f::new_translation(&self.translation)
            * self.rotation.rotation_matrix()
    }
}

impl From<Matrix4f> for Transform {
    fn from(value: Matrix4f) -> Self {
        let translation = Vector3f::new(value[0][3], value[1][3], value[2][3]);
        let scale = Vector3f::new(
            Vector3f::new(value[0][0], value[1][0], value[2][0]).norm(),
            Vector3f::new(value[0][1], value[1][1], value[2][1]).norm(),
            Vector3f::new(value[0][2], value[1][2], value[2][2]).norm(),
        );

        #[rustfmt::skip]
        let rotation_matrix = Matrix4f::with_values([
           value[0][0]/scale.x, value[0][1]/scale.y, value[0][2]/scale.z, 0.0,
           value[1][0]/scale.x, value[1][1]/scale.y, value[1][2]/scale.z, 0.0,
           value[2][0]/scale.x, value[2][1]/scale.y, value[2][2]/scale.z, 0.0,
           0.0, 0.0, 0.0, 1.0,
        ]);

        let rotation = rotation_matrix.into();

        Self {
            translation,
            scale,
            rotation,
        }
    }
}

pub struct TransformCache {
    transform_matrices: HashMap<usize, Matrix4f>,
}

impl TransformCache {
    #[must_use]
    pub fn new() -> Self {
        Self {
            transform_matrices: HashMap::new(),
        }
    }

    pub fn set(&mut self, id: usize, matrix: Matrix4f) {
        self.transform_matrices.insert(id, matrix);
    }

    #[must_use]
    pub fn get(&self, id: usize) -> Matrix4f {
        *self
            .transform_matrices
            .get(&id)
            .unwrap_or(&Matrix4f::identity())
    }
}

impl Default for TransformCache {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            translation: Vector3f::new(0.0, 0.0, 0.0),
            scale: Vector3f::new(1.0, 1.0, 1.0),
            rotation: Quaternion::new(1.0, Vector3f::new(0.0, 0.0, 0.0)),
        }
    }
}
