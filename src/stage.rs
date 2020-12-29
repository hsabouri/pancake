use super::math::{default_triangle, equal_float, Highest, Line, Lowest, Segment, X, Y, Z};
use stl_io::{Triangle, Vertex};

#[derive(Debug, Clone)]
pub struct Stage {
    pub min_height: f32,
    pub max_height: f32,
    // Line, Line, Normal
    pub links: Vec<(Line, Line, [f32; 3])>,
}

pub trait GetStage {
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

        // Selecting in range triangles
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

        let mut links: Vec<(Line, Line, Vertex)> = vec![];

        // Selecting 2 segments of each triangle
        for face in selected.iter() {
            let normal = face.normal;
            let mut vertices = face.vertices.clone();

            // Sorting bottom to top, to make assomptions on the triangle shape
            vertices.sort_by(|a, b| a[Z].partial_cmp(&b[Z]).unwrap());

            let vertices = vertices;
            let a = vertices[0];
            let b = vertices[1];
            let c = vertices[2];

            let segments: Vec<Segment>;

            // Dismissing flat triangles
            if equal_float(a[Z], b[Z]) && equal_float(a[Z], c[Z]) {
                continue;
            } else if equal_float(a[Z], b[Z]) || b[Z] <= min_height {
                segments = vec![
                    Segment {
                        normal,
                        vertices: [a, c],
                    },
                    Segment {
                        normal,
                        vertices: [b, c],
                    },
                    // Dissmissing last segment (0,1) which is flat
                ];
            } else {
                segments = vec![
                    Segment {
                        normal,
                        vertices: [a, b],
                    },
                    Segment {
                        normal,
                        vertices: [a, c],
                    },
                    // Dissmissing last segment (1,2) which won't collide
                ];
            }

            links.push((Line::from(&segments[0]), Line::from(&segments[1]), normal));
        }

        Some(Stage {
            min_height,
            max_height,
            links,
        })
    }
}

#[derive(Debug)]
pub struct StageIterator<'a, T: GetStage> {
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

pub trait IterStages {
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
