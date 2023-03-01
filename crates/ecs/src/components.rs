use std::any::Any;
use glam::*;

pub trait Component {
    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug, Clone)]
pub struct Position {
    pub x : f32,
    pub y: f32,
}

impl Component for Position {

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct Transform {
    pub position: glam::Vec3,
    //pub rotation: glam::Quat,
    pub scale: glam::Vec3,
}

impl Component for Transform {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Transform {
    fn new (pos: Option<glam::Vec3>, /*rot: Option<glam::Quat>*/ scl: Option<glam::Vec3>) -> Self {
        let mut pos_new = glam::Vec3::new(0.0,0.0,0.0);
        //let mut rot_new = glam::Quat::from_mat4()
        let mut scl_new = glam::Vec3::new(0.0,0.0,0.0);
        if let Some(pos) = pos {
            pos_new = pos;
        } 

        // if let Some(rot) = rot {
        //     rot_new = rot;
        // }

        if let Some(scl) = scl {
            scl_new = scl;
        }
        
        Self{position: pos_new, scale: scl_new}
    }
}



// pub struct Velocity {
//     x : f32,
//     y: f32,
// }

// impl Component for Velocity {
    
// }