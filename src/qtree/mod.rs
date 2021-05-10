use std::fmt;

pub struct QuadTree<'a, T> {
    capacity: uint,
    depth: uint,
    max_depth: uint,
    bounds: Bounds,
    elements: Vec<&'a T>,
    children: Option<[Box<QuadTree<'a, T>>, .. 4]>,
}
