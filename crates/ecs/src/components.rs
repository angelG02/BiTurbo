use super::serialization::*;
use erased_serde::Serialize;
use serde_derive::Serialize;
use std::any::Any;
pub trait Component: Serialize {
    fn as_any(&self) -> &dyn Any;
}
#[derive(Serialize, Debug)]
pub struct TransformComponent {
    pub position: BiTurboVec3,
    //pub rotation: glam::Quat,
    pub scale: BiTurboVec3,
}
impl Component for TransformComponent {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl TransformComponent {
    pub fn new(
        pos: Option<BiTurboVec3>,
        /*rot: Option<glam::Quat>*/ scl: Option<BiTurboVec3>,
    ) -> Self {
        let mut pos_new = BiTurboVec3::from(glam::Vec3::new(0.0, 0.0, 0.0));
        //let mut rot_new = glam::Quat::from_mat4()
        let mut scl_new = BiTurboVec3::from(glam::Vec3::new(0.0, 0.0, 0.0));
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

// pub struct Velocity {
//     x : f32,
//     y: f32,
// }

// impl Component for Velocity {

// }
