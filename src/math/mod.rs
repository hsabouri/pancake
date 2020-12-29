use std::convert::From;
use stl_io::{Triangle, Vertex};

mod polygon;

pub use polygon::Polygon;

pub const Z: usize = 2;
pub const Y: usize = 1;
pub const X: usize = 0;

pub fn default_triangle() -> Triangle {
    Triangle {
        normal: [0.0; 3],
        vertices: [[0.0; 3]; 3],
    }
}

pub fn equal_float(a: f32, b: f32) -> bool {
    a <= b + 0.0001 && a >= b - 0.0001
}

pub fn equal_vertices(a: Vertex, b: Vertex) -> bool {
    equal_float(a[0], b[0])
    && equal_float(a[1], b[1])
    && equal_float(a[2], b[2])
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
    pub delta: (f32, f32),
    pub offset: Vertex,
}

impl From<&Segment> for Line {
    fn from(seg: &Segment) -> Self {
        let a = seg.vertices[0];
        let b = seg.vertices[1];
        let h = b[Z] - a[Z];

        Line {
            delta: (
                (b[X] - a[X]) / h,
                (b[Y] - a[Y]) / h,
            ),
            offset: a,
        }
    }
}

pub trait Lowest {
    fn lowest(&self) -> Option<f32>;
}

impl Lowest for stl_io::IndexedMesh {
    fn lowest(&self) -> Option<f32> {
        let first = self.vertices.first()?[Z];
        Some(
            self.vertices
                .iter()
                .fold(first, |acc, v| if v[Z] < acc { v[Z] } else { acc }),
        )
    }
}

pub trait Highest {
    fn highest(&self) -> Option<f32>;
}

impl Highest for stl_io::IndexedMesh {
    fn highest(&self) -> Option<f32> {
        let first = self.vertices.first()?[Z];
        Some(
            self.vertices
                .iter()
                .fold(first, |acc, v| if v[Z] > acc { v[Z] } else { acc }),
        )
    }
}

pub trait Scale {
    fn scale(self, x: f32, y: f32, z: f32) -> Self;
}

impl Scale for stl_io::IndexedMesh {
    fn scale(mut self, x: f32, y: f32, z: f32) -> Self {
        for vertice in self.vertices.iter_mut() {
            vertice[X] = vertice[X] * x;
            vertice[Y] = vertice[Y] * y;
            vertice[Z] = vertice[Z] * z;
        }
        
        self
    }
}

pub trait Homothety {
    fn homothety(self, s: f32) -> Self;
}

impl<T> Homothety for T
where 
    T: Scale
{
    fn homothety(self, s: f32) -> Self {
        self.scale(s, s, s)
    }
}

pub trait Displace {
    fn displace(self, x: f32, y: f32, z: f32) -> Self;
}

impl Displace for stl_io::IndexedMesh {
    fn displace(mut self, x: f32, y: f32, z: f32) -> Self {
        for vertice in self.vertices.iter_mut() {
            vertice[X] = vertice[X] + x;
            vertice[Y] = vertice[Y] + y;
            vertice[Z] = vertice[Z] + z;
        }
        
        self       
    }
}

// TODO: Use Quaternions
pub trait RotateX {
    fn rotate_x(self, theta: f32) -> Self;
}

impl RotateX for stl_io::IndexedMesh {
    fn rotate_x(mut self, theta: f32) -> Self {
        let cos_t = theta.cos();
        let sin_t = theta.sin();

        for vertice in self.vertices.iter_mut() {
            vertice[Y] = cos_t * vertice[Y] - sin_t * vertice[Z];
            vertice[Z] = sin_t * vertice[Y] + cos_t * vertice[Z];
        }

        self
    }
}
pub trait RotateY {
    fn rotate_y(self, theta: f32) -> Self;
}

impl RotateY for stl_io::IndexedMesh {
    fn rotate_y(mut self, theta: f32) -> Self {
        let cos_t = theta.cos();
        let sin_t = theta.sin();

        for vertice in self.vertices.iter_mut() {
            vertice[X] = cos_t * vertice[X] + sin_t * vertice[Z];
            vertice[Z] = cos_t * vertice[Z] - sin_t * vertice[X];
        }

        self
    }
}
pub trait RotateZ {
    fn rotate_z(self, theta: f32) -> Self;
}

impl RotateZ for stl_io::IndexedMesh {
    fn rotate_z(mut self, theta: f32) -> Self {
        let cos_t = theta.cos();
        let sin_t = theta.sin();

        for vertice in self.vertices.iter_mut() {
            vertice[X] = cos_t * vertice[X] - sin_t * vertice[Y];
            vertice[Y] = sin_t * vertice[X] + cos_t * vertice[Y];
        }

        self
    }
}

pub trait Center {
    fn center(self) -> Self;
}

impl Center for stl_io::IndexedMesh {
    fn center(self) -> Self {
        let len = self.vertices.len() as f32;
        let first = self.vertices.iter().next();

        if let Some(first) = first {
            let total = self.vertices.iter().fold(*first, |acc, v| {
                [
                    acc[X] + v[X],
                    acc[Y] + v[Y],
                    acc[Z] + v[Z],
                ]
            });

            let offset = [
                total[X] / len,
                total[Y] / len,
                total[Z] / len,
            ];

            self.displace(offset[X], offset[Y], offset[Z])
        } else {
            self
        }
    }
}