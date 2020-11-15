#[derive(Debug, Clone)]
pub enum Axis {
    X, Y, Z,
}

#[derive(Debug, Clone)]
pub enum Transform {
    Center,
    Rotate(Axis, f32),
    Move(f32, f32, f32),
    Scale(f32, f32, f32),
    Homothety(f32),
}