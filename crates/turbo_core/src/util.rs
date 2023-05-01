use cgmath::{Matrix4, Quaternion, SquareMatrix, Vector3, Vector4};

pub use std::cell::{Ref, RefCell, RefMut};
pub use std::rc::Rc;
pub use std::sync::Arc;
pub use std::sync::{Mutex, MutexGuard};

#[derive(Clone)]
pub struct RcCell<T> {
    value: Rc<RefCell<T>>,
}

impl<T> RcCell<T> {
    pub fn new(value: T) -> Self {
        RcCell {
            value: Rc::new(RefCell::new(value)),
        }
    }

    pub fn strong_count(&self) -> usize {
        Rc::strong_count(&self.value)
    }

    pub fn as_ref(&self) -> Ref<'_, T> {
        self.value.as_ref().borrow()
    }

    pub fn as_mut(&self) -> RefMut<'_, T> {
        self.value.as_ref().borrow_mut()
    }

    pub fn as_ptr(&self) -> *const T {
        RefCell::as_ptr(&self.value)
    }

    pub fn clone(&self) -> RcCell<T> {
        RcCell {
            value: self.value.clone(),
        }
    }
}

#[derive(Clone)]
pub struct ArcMutex<T> {
    value: Arc<Mutex<T>>,
}

impl<T> ArcMutex<T> {
    pub fn new(value: T) -> Self {
        ArcMutex {
            value: Arc::new(Mutex::new(value)),
        }
    }

    pub fn strong_count(&self) -> usize {
        Arc::strong_count(&self.value)
    }

    pub fn as_ref(&self) -> MutexGuard<'_, T> {
        self.value.as_ref().lock().expect("Failed to lock ArcMutex")
    }

    pub fn as_mut(&self) -> MutexGuard<'_, T> {
        self.value.as_ref().lock().expect("Failed to lock ArcMutex")
    }

    pub fn clone(&self) -> ArcMutex<T> {
        ArcMutex {
            value: self.value.clone(),
        }
    }
}

pub fn vec_remove_multiple<T>(vec: &mut Vec<T>, indices: &mut Vec<usize>) {
    indices.sort();

    let mut j: usize = 0;
    for i in indices.iter() {
        vec.remove(i - j);
        j += 1;
    }
}

pub fn quaternion_to_matrix(quaternion: &Quaternion<f32>) -> Matrix4<f32> {
    let mut mat4: Matrix4<f32> = SquareMatrix::identity();

    let quat = Vector4::<f32>::new(quaternion.v.x, quaternion.v.y, quaternion.v.z, quaternion.s);

    mat4.x[0 - 0] = 1.0 - 2.0 * quat.y * quat.y - 2.0 * quat.z * quat.z;
    mat4.x[1 - 0] = 2.0 * quat.x * quat.y - 2.0 * quat.w * quat.z;
    mat4.x[2 - 0] = 2.0 * quat.x * quat.z + 2.0 * quat.w * quat.y;
    mat4.y[4 - 4] = 2.0 * quat.x * quat.y + 2.0 * quat.w * quat.z;
    mat4.y[5 - 4] = 1.0 - 2.0 * quat.x * quat.x - 2.0 * quat.z * quat.z;
    mat4.y[6 - 4] = 2.0 * quat.y * quat.z - 2.0 * quat.w * quat.x;
    mat4.z[8 - 8] = 2.0 * quat.x * quat.z - 2.0 * quat.w * quat.y;
    mat4.z[9 - 8] = 2.0 * quat.y * quat.z + 2.0 * quat.w * quat.x;
    mat4.z[10 - 8] = 1.0 - 2.0 * quat.x * quat.x - 2.0 * quat.y * quat.y;

    mat4
}

pub fn up() -> Vector3<f32> {
    Vector3::new(0.0, 1.0, 0.0)
}

pub fn right() -> Vector3<f32> {
    Vector3::new(1.0, 0.0, 0.0)
}

pub fn forward() -> Vector3<f32> {
    Vector3::new(0.0, 0.0, -1.0)
}
