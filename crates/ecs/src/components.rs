use std::any::Any;

pub trait Component {
    fn asd(&self);
    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug, Clone)]
pub struct Position {
    pub x : f32,
    pub y: f32,
}

impl Component for Position {
    fn asd(&self) {
        todo!()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// pub struct Velocity {
//     x : f32,
//     y: f32,
// }

// impl Component for Velocity {
    
// }