use super::{
    stage::Stage,
    math::{
        Segment,
        Polygon,
        X, Y, Z,
    },
};

#[derive(Debug, Clone)]
pub struct Slice {
    pub height: f32,
    pub polygons: Vec<Polygon>,
}

pub trait GetSlice {
    fn get_slice(&self, height: f32) -> Option<Slice>;
}

impl GetSlice for Stage {
    fn get_slice(&self, height: f32) -> Option<Slice> {
        if height < self.min_height || height > self.max_height {
            return None;
        }

        let segments: Vec<Segment> = self
            .links
            .iter()
            .map(|(line_a, line_b, normal)| {
                Segment {
                    normal: normal.clone(),
                    vertices: [
                        [
                            line_a.offset[X] + line_a.delta.0 * (height - line_a.offset[Z]),
                            line_a.offset[Y] + line_a.delta.1 * (height - line_a.offset[Z]),
                            height,
                        ],
                        [
                            line_b.offset[X] + line_b.delta.0 * (height - line_b.offset[Z]),
                            line_b.offset[Y] + line_b.delta.1 * (height - line_b.offset[Z]),
                            height,
                        ]
                    ]
                }
            })
            .collect();

        Some(
            Slice {
                height,
                polygons: Polygon::build(segments),
            }
        )
    }
}

pub struct SliceIterator<'a, T>
where
    T: Iterator<Item = Stage>,
{
    current: Stage,
    inner: &'a mut T,
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
            self.current = self.inner.next()?;
        }

        self.last_height = height;
        self.current.get_slice(height)
    }
}

pub trait IterSlices {
    type Inner: Iterator<Item = Stage>;
    fn iter_slices(&mut self, step: f32) -> Option<SliceIterator<Self::Inner>>;
}

impl<T> IterSlices for T
where
    T:Iterator<Item = Stage>
{
    type Inner = T;
    fn iter_slices(&mut self, step: f32) -> Option<SliceIterator<Self::Inner>> {
        let current = self.next()?;

        Some(SliceIterator {
            last_height: current.min_height,
            current: current,
            inner: self,
            step,
        })
    }
}