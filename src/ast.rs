#[derive(Debug, Clone)]
pub enum Axis {
    X, Y, Z,
}

#[derive(Debug, Clone)]
pub enum Transform {
    Center,
    Rotate(Axis, f64),
    Move(f64, f64, f64),
    Scale(f64, f64, f64),
    Homothety(f64),
}