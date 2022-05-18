use std::{convert::From, f64::EPSILON};
use stl_io::{Triangle, Vector, Vertex};

mod polygon;

pub use polygon::Polygon;

pub const Z: usize = 2;
pub const Y: usize = 1;
pub const X: usize = 0;

pub fn default_triangle() -> Triangle {
    Triangle {
        normal: Vector::new([0.0; 3]),
        vertices: [Vector::new([0.0; 3]); 3],
    }
}

pub fn equal_float(a: f64, b: f64) -> bool {
    a <= b + EPSILON && a >= b - EPSILON
}

pub fn equal_vertices(a: Vertex, b: Vertex) -> bool {
    equal_float(a[0], b[0]) && equal_float(a[1], b[1]) && equal_float(a[2], b[2])
}

#[derive(Debug, Clone)]
pub struct Segment {
    pub normal: Vertex,
    pub vertices: [Vertex; 2],
}

impl Segment {
    pub fn reverse(&mut self) {
        let (a, b) = (self.vertices[1], self.vertices[0]);

        self.vertices[0] = a;
        self.vertices[1] = b;
    }

    pub fn correct_direction(&mut self) {
        let a = self.vertices[0];
        let b = self.vertices[1];
        let c = self.normal;

        let det = (b[X] - a[X]) * (c[Y] - a[Y]) - (b[Y] - a[Y]) * (c[X] - a[X]);

        if det < 0.0 {
            self.vertices[0] = b;
            self.vertices[1] = a;
        }
    }
}

#[derive(Debug, Clone)]
pub struct Line {
    pub delta: (f64, f64),
    pub offset: Vertex,
}

impl From<&Segment> for Line {
    fn from(seg: &Segment) -> Self {
        let a = seg.vertices[0];
        let b = seg.vertices[1];
        let h = b[Z] - a[Z];

        Line {
            delta: ((b[X] - a[X]) / h, (b[Y] - a[Y]) / h),
            offset: a,
        }
    }
}

pub trait Lowest {
    fn lowest(&self) -> Option<f64>;
}

impl Lowest for stl_io::IndexedMesh {
    fn lowest(&self) -> Option<f64> {
        let first = self.vertices.first()?[Z];
        Some(
            self.vertices
                .iter()
                .fold(first, |acc, v| if v[Z] < acc { v[Z] } else { acc }),
        )
    }
}

pub trait Highest {
    fn highest(&self) -> Option<f64>;
}

impl Highest for stl_io::IndexedMesh {
    fn highest(&self) -> Option<f64> {
        let first = self.vertices.first()?[Z];
        Some(
            self.vertices
                .iter()
                .fold(first, |acc, v| if v[Z] > acc { v[Z] } else { acc }),
        )
    }
}

pub trait Scale {
    fn scale(self, x: f64, y: f64, z: f64) -> Self;
}

impl Scale for stl_io::IndexedMesh {
    fn scale(mut self, x: f64, y: f64, z: f64) -> Self {
        for vertice in self.vertices.iter_mut() {
            let x = vertice[X] * x;
            let y = vertice[Y] * y;
            let z = vertice[Z] * z;

            *vertice = Vertex::new([x, y, z])
        }

        self
    }
}

pub trait Homothety {
    fn homothety(self, s: f64) -> Self;
}

impl<T> Homothety for T
where
    T: Scale,
{
    fn homothety(self, s: f64) -> Self {
        self.scale(s, s, s)
    }
}

pub trait Displace {
    fn displace(self, x: f64, y: f64, z: f64) -> Self;
}

impl Displace for stl_io::IndexedMesh {
    fn displace(mut self, x: f64, y: f64, z: f64) -> Self {
        for vertice in self.vertices.iter_mut() {
            let x = vertice[X] + x;
            let y = vertice[Y] + y;
            let z = vertice[Z] + z;

            *vertice = Vertex::new([x, y, z])
        }

        self
    }
}

// TODO: Use Quaternions
pub trait RotateX {
    fn rotate_x(self, theta: f64) -> Self;
}

impl RotateX for stl_io::IndexedMesh {
    fn rotate_x(mut self, theta: f64) -> Self {
        let cos_t = theta.cos();
        let sin_t = theta.sin();

        for vertice in self.vertices.iter_mut() {
            let y = cos_t * vertice[Y] - sin_t * vertice[Z];
            let z = sin_t * vertice[Y] + cos_t * vertice[Z];

            *vertice = Vertex::new([vertice[X], y, z])
        }

        self
    }
}
pub trait RotateY {
    fn rotate_y(self, theta: f64) -> Self;
}

impl RotateY for stl_io::IndexedMesh {
    fn rotate_y(mut self, theta: f64) -> Self {
        let cos_t = theta.cos();
        let sin_t = theta.sin();

        for vertice in self.vertices.iter_mut() {
            let x = cos_t * vertice[X] + sin_t * vertice[Z];
            let z = cos_t * vertice[Z] - sin_t * vertice[X];

            *vertice = Vertex::new([x, vertice[Y], z])
        }

        self
    }
}
pub trait RotateZ {
    fn rotate_z(self, theta: f64) -> Self;
}

impl RotateZ for stl_io::IndexedMesh {
    fn rotate_z(mut self, theta: f64) -> Self {
        let cos_t = theta.cos();
        let sin_t = theta.sin();

        for vertice in self.vertices.iter_mut() {
            let x = cos_t * vertice[X] - sin_t * vertice[Y];
            let y = sin_t * vertice[X] + cos_t * vertice[Y];

            *vertice = Vertex::new([x, y, vertice[Z]])
        }

        self
    }
}

pub trait Center {
    fn center(self) -> Self;
}

impl Center for stl_io::IndexedMesh {
    fn center(self) -> Self {
        let len = self.vertices.len() as f64;
        let first = self.vertices.iter().next();

        if let Some(first) = first {
            let total = self.vertices.iter().fold(*first, |acc, v| {
                Vector::new([acc[X] + v[X], acc[Y] + v[Y], acc[Z] + v[Z]])
            });

            let offset = [total[X] / len, total[Y] / len, total[Z] / len];

            self.displace(offset[X], offset[Y], offset[Z])
        } else {
            self
        }
    }
}
