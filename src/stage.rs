use stl_io::{Triangle, Vertex};
use super::math::{
    Line,
    Segment,
    X, Y, Z,
    equal_float,
    default_triangle,
    Lowest,
    Highest,
};

#[derive(Debug, Clone)]
pub struct Stage {
    pub min_height: f32,
    pub max_height: f32,
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

        for face in selected.iter() {
            let normal = face.normal;
            let mut vertices = face.vertices.clone();

            // Sorting to make assomptions on the triangle shape
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

            links.push((
                Line::from(&segments[0]),
                Line::from(&segments[1]),
                normal)
            );
        }

        Some(Stage {
            min_height,
            max_height,
            links,
        })
    }
}


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

