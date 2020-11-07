use std::convert::{Into, From};
use stl_io::{Triangle, Vertex};

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
    pub normal: [f32; 3],
    pub vertices: [Vertex; 2],
}

#[derive(Debug, Clone)]
pub struct Line {
    pub delta: (f32, f32),
    pub offset: Vertex,
}

impl From<&Segment> for Line {
    fn from(seg: &Segment) -> Self {
        Line {
            delta: (
                (seg.vertices[1][X] - seg.vertices[0][X])
                    / (seg.vertices[1][Z] - seg.vertices[0][Z]),
                (seg.vertices[1][Y] - seg.vertices[0][Y])
                    / (seg.vertices[1][Z] - seg.vertices[0][Z]),
            ),
            offset: seg.vertices[0],
        }
    }
}

#[derive(Debug, Clone)]
pub struct Polygon (Vec<Segment>);

impl Polygon {
    pub fn new(into: Vec<Segment>) -> Self {
        Self (into)
    }

    fn find_and_assign(poly_vec: &mut Vec<Self>, seg: &Segment) {
        // TODO: polygon fuse algo
        unimplemented!()
    }

    pub fn build(input: Vec<Segment>) -> Vec<Self> {
        let mut ret: Vec<Self> = vec![];

        for seg in input.iter() {
            Self::find_and_assign(&mut ret, seg);
        }

        ret
    }
}

impl Into<Vec<Segment>> for Polygon {
    fn into(self) -> Vec<Segment> {
        self.0
    }
}

impl IntoIterator for Polygon {
    type Item = Segment;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
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

