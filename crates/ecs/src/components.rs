use super::serialization::*;
use erased_serde::serialize_trait_object;
use serde_derive::Serialize;
//use serde::Serialize;
//use serde::Serialize;
use std::{any::Any, fmt::Debug};
pub trait Component: erased_serde::Serialize + Debug {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}
serialize_trait_object!(Component);
#[derive(Serialize, Debug)]
pub struct TransformComponent {
    pub position: SerializedVec3,
    //pub rotation: glam::Quat,
    pub scale: SerializedVec3,
}

impl Component for TransformComponent {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl TransformComponent {
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

    pub fn serialize_transform(&self) {
        let test = serde_json::to_string(&self);

        println!("Serialized data {:?}", test);
    }
}
