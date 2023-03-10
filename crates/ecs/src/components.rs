use super::serialization::*;

use serde_derive::{Deserialize, Serialize};
use std::{any::Any, fmt::Debug};
use typetag;
#[typetag::serde(tag = "type")]
pub trait Component: erased_serde::Serialize + Debug + Any {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}
//serialize_trait_object!(Component);

// impl<T: Any + Send + Sync + Debug + erased_serde::Serialize> Component for T {
//     fn as_any(&self) -> &dyn Any {
//         self
//     }

//     fn as_any_mut(&mut self) -> &mut dyn Any {
//         self
//     }
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct TransformComponent {
    pub position: SerializedVec3,
    //pub rotation: glam::Quat,
    pub scale: SerializedVec3,
}

#[typetag::serde]
impl Component for TransformComponent {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl TransformComponent {
    /// Creates a new `Transform` with the given position and scale.
    ///
    /// The `pos` argument specifies the initial position of the transform,
    /// which can be provided as a `SerializedVec3` option. If `pos` is `None`,
    /// the default position of the new transform will be (0.0, 0.0, 0.0).
    ///
    /// The `scl` argument specifies the initial scale of the transform, also as
    /// a `SerializedVec3` option. If `scl` is `None`, the default scale of the
    /// new transform will be (0.0, 0.0, 0.0).
    ///
    /// The `rot` argument is currently commented out and not used, since there
    /// is no implementation for setting the rotation of the transform. If you
    /// need to set the rotation, you will need to modify this function or add
    /// another function to the `Transform` struct.
    ///
    /// Returns the new `Transform` with the specified position and scale.
    pub fn new(
        pos: Option<SerializedVec3>,
        /*rot: Option<glam::Quat>*/ scl: Option<SerializedVec3>,
    ) -> Self {
        let mut pos_new = SerializedVec3::from(glam::Vec3::new(0.0, 0.0, 0.0));
        //let mut rot_new = glam::Quat::from_mat4()
        let mut scl_new = SerializedVec3::from(glam::Vec3::new(0.0, 0.0, 0.0));
        if let Some(pos) = pos {
            pos_new = pos;
        }

        // if let Some(rot) = rot {
        //     rot_new = rot;
        // }

        if let Some(scl) = scl {
            scl_new = scl;
        }

        Self {
            position: pos_new,
            scale: scl_new,
        }
    }
}
