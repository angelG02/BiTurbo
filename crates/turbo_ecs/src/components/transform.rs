use bevy_ecs::prelude::*;
use cgmath::{Matrix4, Quaternion, SquareMatrix, Vector3};

use turbo_core::util::*;

#[derive(Component, Debug, PartialEq, Clone, Copy)]
pub struct Transform {
    translation: Vector3<f32>,
    rotation: Quaternion<f32>,
    scale: Vector3<f32>,

    // TODO: I believe we can detect when a component in the world has changed since last frame
    // so we dont have to calculate the model matrix every frame and we don't need to store it
    // here either since that will take a lot of space... but recalulate it when changed?
    model_matrix: Matrix4<f32>,
    model_matrix_inv_trans: Matrix4<f32>,
    model_matrix_dirty: bool,
}

impl Transform {
    //
    pub fn new(translation: Vector3<f32>, rotation: Quaternion<f32>, scale: Vector3<f32>) -> Self {
        Transform {
            translation,
            rotation,
            scale,
            model_matrix: SquareMatrix::identity(), // uhm idk if this will work but I leave for now :))
            model_matrix_inv_trans: SquareMatrix::identity(),
            model_matrix_dirty: true,
        }
    }

    pub fn get_translation(&self) -> &Vector3<f32> {
        &self.translation
    }

    pub fn get_rotation(&self) -> &Quaternion<f32> {
        &self.rotation
    }

    pub fn get_scale(&self) -> &Vector3<f32> {
        &self.scale
    }

    pub fn set_translation(&mut self, translation: &Vector3<f32>) {
        self.translation = translation.clone();
        self.model_matrix_dirty = true;
    }

    pub fn set_rotation(&mut self, rotation: &Quaternion<f32>) {
        self.rotation = rotation.clone();
        self.model_matrix_dirty = true;
    }

    pub fn set_scale(&mut self, scale: &Vector3<f32>) {
        self.scale = scale.clone();
        self.model_matrix_dirty = true;
    }

    pub fn translate(&mut self, translation: &Vector3<f32>) {
        self.set_translation(&(self.translation + translation));
    }

    pub fn rotate(&mut self, rotation: &Quaternion<f32>) {
        self.set_rotation(&(self.rotation * rotation));
    }

    pub fn scale(&mut self, scale: &Vector3<f32>) {
        self.set_scale(&(self.scale + scale));
    }

    pub fn get_matrix(&mut self, invert: bool) -> &Matrix4<f32> {
        if self.model_matrix_dirty {
            self.recalculate_matrix(invert);
        }

        &self.model_matrix
    }

    fn recalculate_matrix(&mut self, invert: bool) {
        self.model_matrix = if invert {
            Matrix4::from_translation(-self.translation)
                * quaternion_to_matrix(&self.rotation)
                * Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z)
        } else {
            Matrix4::from_translation(self.translation)
                * quaternion_to_matrix(&self.rotation)
                * Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z)
        };
        self.model_matrix_inv_trans = self.model_matrix.clone().invert().unwrap();
        self.model_matrix_inv_trans.transpose_self();
        self.model_matrix_dirty = false;
    }
}

impl Default for Transform {
    fn default() -> Self {
        Transform {
            translation: Vector3::new(0.0, 0.0, 0.0),
            rotation: Quaternion::new(1.0, 0.0, 0.0, 0.0),
            scale: Vector3::new(1.0, 1.0, 1.0),
            model_matrix: SquareMatrix::identity(),
            model_matrix_inv_trans: SquareMatrix::identity(),
            model_matrix_dirty: true,
        }
    }
}
