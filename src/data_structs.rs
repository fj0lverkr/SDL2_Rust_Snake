pub struct Position2D {
    pub x: i32,
    pub y: i32,
}

pub struct Position3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Position2D {
    pub fn new(x: i32, y: i32) -> Position2D {
        Position2D { x, y }
    }
}

impl Position3D {
    pub fn new(x: i32, y: i32, z: i32) -> Position3D {
        Position3D { x, y, z }
    }
}
