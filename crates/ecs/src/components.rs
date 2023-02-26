pub trait Component {
    fn asd(&self);
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
}

// pub struct Velocity {
//     x : f32,
//     y: f32,
// }

// impl Component for Velocity {
    
// }