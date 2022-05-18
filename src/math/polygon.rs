use std::{convert::Into, ops::Deref};

use super::{equal_vertices, Segment};

#[derive(Debug, Copy, Clone, PartialEq)]
enum Push {
    Back,
    Front,
}

#[derive(Debug, Clone)]
pub struct Polygon(Vec<Segment>);

impl Polygon {
    pub fn new(into: Vec<Segment>) -> Self {
        Self(into)
    }

    fn reverse(&mut self) {
        self.0.reverse();

        self.0.iter_mut().for_each(|s| s.reverse());
    }

    fn merge_polys(mut a: Self, mut b: Self, action_a: Push, action_b: Push) -> Self {
        match (action_a, action_b) {
            (Push::Back, Push::Back) => b.reverse(),
            (Push::Front, Push::Front) => a.reverse(),
            (Push::Front, Push::Back) => {
                a.reverse();
                b.reverse();
            }
            _ => (),
        };

        a.0.extend(b.0.into_iter());
        a
    }

    fn merge_matches(poly_vec: &mut Vec<Self>, mut poly_ids: Vec<(usize, Push)>) {
        // Sort in reverse order to avoid offsetting ids when removing in the vector
        poly_ids.sort_by(|(i, _), (other, _)| other.cmp(i));

        let mut new_poly_vec = vec![];
        let mut poly_ids = poly_ids.iter();
        let action = poly_ids.next();

        if let Some((initial_i, action)) = action {
            let mut last_action = action;
            let mut last_poly = poly_vec.remove(*initial_i);

            for (i, action) in poly_ids {
                let poly = poly_vec.remove(*i);

                last_poly = Self::merge_polys(last_poly, poly, *last_action, *action);
                last_action = action;
            }

            new_poly_vec.push(last_poly);
        }

        poly_vec.extend(new_poly_vec.into_iter());
    }

    fn find_and_assign(poly_vec: &mut Vec<Self>, seg: Segment) {
        let poly_ids: Vec<(usize, Push)> = poly_vec
            .iter()
            .enumerate()
            .filter_map(|(i, poly)| {
                let belongs = poly.0.iter().find_map(|poly_seg| {
                    if equal_vertices(seg.vertices[0], poly_seg.vertices[0])
                        || equal_vertices(seg.vertices[1], poly_seg.vertices[0])
                    {
                        Some(Push::Front)
                    } else if equal_vertices(seg.vertices[0], poly_seg.vertices[1])
                        || equal_vertices(seg.vertices[1], poly_seg.vertices[1])
                    {
                        Some(Push::Back)
                    } else {
                        None
                    }
                });

                if let Some(push) = belongs {
                    Some((i, push))
                } else {
                    None
                }
            })
            .collect();

        if let Some((i, action)) = poly_ids.first() {
            if *action == Push::Back {
                poly_vec.get_mut(*i).unwrap().0.push(seg.clone());
            } else if *action == Push::Front {
                poly_vec.get_mut(*i).unwrap().0.insert(0, seg.clone());
            }

            Self::merge_matches(poly_vec, poly_ids);
        } else {
            poly_vec.push(Polygon::new(vec![seg.clone()]));
        }
    }

    pub fn build(input: Vec<Segment>) -> Vec<Self> {
        let mut ret: Vec<Self> = vec![];

        for mut seg in input.into_iter() {
            seg.correct_direction();

            Self::find_and_assign(&mut ret, seg);
        }

        eprintln!("Number of polys : {}", ret.len());

        ret
    }
}

impl Deref for Polygon {
    type Target = Vec<Segment>;

    fn deref(&self) -> &Self::Target {
        &self.0
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
