use crate::TransformComponent;

pub trait System {
    fn update(&mut self);
}

pub struct MovementTestSystem {
    pub transform: TransformComponent,
    pub velocity: f32,
}

impl System for MovementTestSystem {
    fn update(&mut self) {
        //println!("IS it updating ? {:?}", self.transform.position.x);
        self.transform.position.x += self.velocity;
    }
}

impl MovementTestSystem {
    pub fn new() -> Self {
        Self {
            transform: TransformComponent::new(None, None),
            velocity: 3.0,
        }
    }
}
