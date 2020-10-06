use std::env;
use std::fs::OpenOptions;
use stl_io::read_stl;

use stl_io::{Vertex, IndexedTriangle, Triangle};

const Z: usize = 2;
const Y: usize = 1;
const X: usize = 0;

struct Segment {
    pub normal: [f32; 2],
    pub vertices: [Vertex; 2],
}

struct HeightSegment {
    pub delta: (f32, f32),
    pub limit: (f32, f32),
}

type Polygon = Vec<Segment>;

trait MeshSlice {
    fn get_mesh_slice(&self, height: f32) -> Polygon;
}

impl MeshSlice for stl_io::IndexedMesh {
    fn get_mesh_slice(&self, height: f32) -> Polygon {
        let segments: Vec<Segment> = self.faces.iter().filter_map(|face| {
            let mut is_above = false;
            let mut is_below = false;

            for vertex_id in face.vertices.iter() {
                let vertex = self.vertices[*vertex_id];

                if vertex[Z] >= height {
                    is_above = true;
                }
                if vertex[Z] <= height {
                    is_below = true;
                }
            }

            if !is_above || !is_below {
                return None
            }

            // TODO: create segment here
            unimplemented!()
        }).collect();

        // TODO: link polygon
        unimplemented!()
    }
}

struct Stage {
    pub height: f32,
    pub polygon: Polygon,
}

struct InterStages {
    pub links: Vec<HeightSegment>,
}

trait GetStage {
    fn get_stage(&self, min_height: f32) -> Option<Stage>;
}

struct StageIterator<'a, T: GetStage> {
    inner: &'a T,
    last_height: f32,
}

impl<'a, T: GetStage> Iterator for StageIterator<'a, T> {
    type Item = Stage;

    fn next(&mut self) -> Option<Stage> {
        let stage = self.inner.get_stage(self.last_height)?;
        self.last_height = stage.height;

        Some(stage)
    }
}

impl GetStage for stl_io::IndexedMesh {
    fn get_stage(&self, min_height: f32) -> Option<Stage> {
        let mut current_height = self.vertices.get(0)?[Z];

        // Find top limit of slice
        for vertex in self.vertices.iter() {
            if vertex[Z] <= current_height && vertex[Z] > min_height {
                current_height = vertex[Z];
            }
        }

        let height = current_height;

        let polygon = self.get_mesh_slice(height);

        Some(Stage {
            height: height,
            polygon: polygon,
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut file = OpenOptions::new().read(true).open(args[1].clone()).unwrap();
    let stl = read_stl(&mut file).unwrap();

}
 