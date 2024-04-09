pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64
}

pub struct Vector2 {
    x: f64,
    y: f64
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        return Vector3 {x, y, z}
    }
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Vector2 {
        return Vector2 {x, y}
    }
    
}