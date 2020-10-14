use std::env;
use std::fs::OpenOptions;
use stl_io::read_stl;

use stl_io::{IndexedTriangle, Triangle, Vertex};

const Z: usize = 2;
const Y: usize = 1;
const X: usize = 0;

fn default_triangle() -> Triangle {
    Triangle {
        normal: [0.0; 3],
        vertices: [[0.0; 3]; 3],
    }
}

fn equal_float(a: f32, b: f32) -> bool {
    a <= b + 0.0001 && a >= b - 0.0001
}

struct Segment {
    pub normal: [f32; 3],
    pub vertices: [Vertex; 2],
}

#[derive(Debug)]
struct Line {
    pub delta: (f32, f32),
    pub offset: Vertex,
}

type Polygon = Vec<Segment>;

#[derive(Debug)]
struct Stage {
    pub min_height: f32,
    pub max_height: f32,
    pub links: Vec<Line>,
}

impl Stage {
    pub fn get_slice(&self, height: f32) -> Option<Slice> {
        if height < self.min_height || height > self.max_height {
            return None;
        }

        let vertices: Vec<Vertex> = self
            .links
            .iter()
            .map(|line| {
                [
                    line.offset[X] + line.delta.0 * (height - line.offset[Z]),
                    line.offset[Y] + line.delta.1 * (height - line.offset[Z]),
                    height,
                ]
            })
            .collect();

        unimplemented!()
    }
}

trait GetStage {
    fn get_stage(&self, min_height: f32) -> Option<Stage>;
}

impl GetStage for stl_io::IndexedMesh {
    fn get_stage(&self, min_height: f32) -> Option<Stage> {
        let highest = self.highest()?;
        let mut current_height = highest;

        if equal_float(min_height, highest) {
            return None;
        }

        // Find top limit of stage
        for vertex in self.vertices.iter() {
            if vertex[Z] <= current_height && vertex[Z] > min_height {
                current_height = vertex[Z];
            }
        }

        let max_height = current_height;

        let selected: Vec<Triangle> = self
            .faces
            .iter()
            .filter_map(|i_face| {
                let mut is_above = false;
                let mut is_below = false;

                let mut face = default_triangle();
                face.normal = i_face.normal;

                for (i, vertex_id) in i_face.vertices.iter().enumerate() {
                    let vertex = self.vertices[*vertex_id];

                    face.vertices[i] = vertex;

                    if vertex[Z] >= max_height {
                        is_above = true;
                    }
                    if vertex[Z] <= min_height {
                        is_below = true;
                    }
                }

                if !is_above || !is_below {
                    return None;
                }

                Some(face)
            })
            .collect();

        let mut links: Vec<Line> = vec![];

        for face in selected.iter() {
            let normal = face.normal;
            let mut vertices = face.vertices.clone();

            // Very usefull to make assomptions on the triangle shape
            vertices.sort_by(|a, b| a[Z].partial_cmp(&b[Z]).unwrap());

            let segments: Vec<Segment>;

            // Dissmissing flat triangles
            if equal_float(vertices[0][Z], vertices[1][Z])
                && equal_float(vertices[0][Z], vertices[2][Z])
            {
                continue;
            } else if equal_float(vertices[0][Z], vertices[1][Z]) {
                segments = vec![
                    Segment {
                        normal,
                        vertices: [vertices[0], vertices[2]],
                    },
                    Segment {
                        normal,
                        vertices: [vertices[1], vertices[2]],
                    },
                    // Dissmissing last segment (0,1) which is flat
                ];
            } else {
                segments = vec![
                    Segment {
                        normal,
                        vertices: [vertices[0], vertices[1]],
                    },
                    Segment {
                        normal,
                        vertices: [vertices[0], vertices[2]],
                    },
                    // Dissmissing last segment (1,2) which won't collide
                ];
            }

            for segment in segments.iter() {
                links.push(Line {
                    delta: (
                        (segment.vertices[1][X] - segment.vertices[0][X])
                            / (segment.vertices[1][Z] - segment.vertices[0][Z]),
                        (segment.vertices[1][Y] - segment.vertices[0][Y])
                            / (segment.vertices[1][Z] - segment.vertices[0][Z]),
                    ),
                    offset: segment.vertices[0],
                })
            }
        }

        Some(Stage {
            min_height,
            max_height,
            links,
        })
    }
}

trait Lowest {
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

trait Highest {
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

struct StageIterator<'a, T: GetStage> {
    inner: &'a T,
    last_height: f32,
}

impl<'a, T: GetStage> Iterator for StageIterator<'a, T> {
    type Item = Stage;

    fn next(&mut self) -> Option<Stage> {
        let stage = self.inner.get_stage(self.last_height)?;
        self.last_height = stage.max_height;

        Some(stage)
    }
}

trait IterStages {
    type Inner: GetStage;
    fn iter_stages(&self) -> Option<StageIterator<Self::Inner>>;
}

impl<T: GetStage + Lowest> IterStages for T {
    type Inner = T;
    fn iter_stages(&self) -> Option<StageIterator<Self::Inner>> {
        Some(StageIterator {
            inner: self,
            last_height: self.lowest()?,
        })
    }
}

struct Slice {
    pub height: f32,
    pub polygon: Polygon,
}

struct SliceIterator<'a, T>
where
    T: Iterator<Item = Stage>,
{
    current: &'a Stage,
    inner: &'a T,
    last_height: f32,
    step: f32,
}

impl<'a, T> Iterator for SliceIterator<'a, T>
where
    T: Iterator<Item = Stage>,
{
    type Item = Slice;

    fn next(&mut self) -> Option<Slice> {
        let height = self.last_height + self.step;

        if height > self.current.max_height {
            self.current = &self.inner.next()?;
        }

        self.current.get_slice(height)
    }
}

trait IterSlices {
    type Inner: Iterator<Item = Stage>;
    fn iter_slices(&self, step: f32) -> Option<SliceIterator<Self::Inner>>;
}

impl<T: Iterator<Item = Stage>> IterSlices for T {
    type Inner = T;
    fn iter_slices(&self, step: f32) -> Option<SliceIterator<Self::Inner>> {
        let current = self.next()?;

        Some(SliceIterator {
            current: &current,
            inner: self,
            last_height: current.min_height,
            step,
        })
    }
}

fn main() -> Result<(), ()> {
    let args: Vec<String> = env::args().collect();

    let mut file = OpenOptions::new().read(true).open(args[1].clone()).unwrap();
    let stl = read_stl(&mut file).unwrap();

    let slices: Vec<Slice> = stl.iter_stages().ok_or(())?.iter_slices(0.2)?.collect();

    Ok(())
}
