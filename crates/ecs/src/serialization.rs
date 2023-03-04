use glam::Vec3;
use serde::ser::{Serialize, SerializeStruct};
#[derive(Debug)]
pub struct BiTurboVec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<Vec3> for BiTurboVec3 {
    fn from(value: Vec3) -> Self {
        Self {
            x: value.x,
            y: (value.y),
            z: (value.z),
        }
    }
}

impl Into<Vec3> for BiTurboVec3 {
    fn into(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
}

impl Serialize for BiTurboVec3 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("BiTurboVec3", 3)?;

        s.serialize_field("x", &self.x)?;
        s.serialize_field("y", &self.y)?;
        s.serialize_field("z", &self.z)?;
        s.end()
    }
}
