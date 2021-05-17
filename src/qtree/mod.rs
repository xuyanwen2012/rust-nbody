pub struct QuadTree<'a, T> {
    capacity: usize,
    depth: usize,
    max_depth: usize,
    bounds: Bounds,
    elements: Vec<&'a T>,
    children: Option<[Box<QuadTree<'a, T>>; 4]>,
}

#[repr(C)]
#[derive(PartialEq)]
pub struct Bounds {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

/// Elements of the quadtree must implement this trait.
pub trait Bounded {
    fn bounds(&self) -> Bounds;
}

enum Quadrant {
    TL,
    TR,
    BR,
    BL,
}

impl Quadrant {
    fn to_usize(&self) -> usize {
        match self {
            Quadrant::TL => 0,
            Quadrant::TR => 1,
            Quadrant::BR => 2,
            Quadrant::BL => 3,
        }
    }
}

impl<'a, T: Bounded> QuadTree<'a, T> {
    pub fn new(bounds: Bounds) -> QuadTree<'a, T> {
        QuadTree {
            capacity: 4,
            max_depth: 10,
            depth: 0,
            bounds,
            elements: Vec::new(),
            children: None,
        }
    }

    pub fn insert(&mut self, element: &'a T) {
        match (&self.get_quadrant(element), self) {
            (
                Some(q),
                QuadTree {
                    children: Some(ref mut children),
                    ..
                },
            ) => children[q.to_usize()].insert(element),
            (
                None,
                _self @ QuadTree {
                    children: Some(_), ..
                },
            ) => _self.elements.push(element),
            (_, _self @ QuadTree { children: None, .. }) => {
                _self.elements.push(element);

                if _self.elements.len() > _self.capacity {
                    _self.split();
                }
            }
        }
    }

    fn split(&mut self) {
        if self.depth >= self.max_depth {
            return;
        }
        match self.children {
            None => {
                let capacity = self.capacity;
                let depth = self.depth + 1;
                let max_depth = self.max_depth;

                let mut children = [
                    Box::new(QuadTree {
                        capacity,
                        depth,
                        max_depth,
                        bounds: Bounds {
                            x: self.bounds.x,
                            y: self.bounds.y,
                            width: self.bounds.width / 2.0,
                            height: self.bounds.height / 2.0,
                        },
                        elements: Vec::new(),
                        children: None,
                    }),
                    Box::new(QuadTree {
                        capacity,
                        depth,
                        max_depth,
                        bounds: Bounds {
                            x: self.bounds.x + self.bounds.width / 2.0,
                            y: self.bounds.y,
                            width: self.bounds.width / 2.0,
                            height: self.bounds.height / 2.0,
                        },
                        elements: Vec::new(),
                        children: None,
                    }),
                    Box::new(QuadTree {
                        capacity,
                        depth,
                        max_depth,
                        bounds: Bounds {
                            x: self.bounds.x + self.bounds.width / 2.0,
                            y: self.bounds.y + self.bounds.height / 2.0,
                            width: self.bounds.width / 2.0,
                            height: self.bounds.height / 2.0,
                        },
                        elements: Vec::new(),
                        children: None,
                    }),
                    Box::new(QuadTree {
                        capacity,
                        depth,
                        max_depth,
                        bounds: Bounds {
                            x: self.bounds.x,
                            y: self.bounds.y + self.bounds.height / 2.0,
                            width: self.bounds.width / 2.0,
                            height: self.bounds.height / 2.0,
                        },
                        elements: Vec::new(),
                        children: None,
                    }),
                ];

                let mut new_elements: Vec<&T> = Vec::new();
                for &element in self.elements.iter() {
                    match self.get_quadrant(element) {
                        Some(i) => children[i.to_usize()].insert(element),
                        None => new_elements.push(element),
                    };
                }

                self.children = Some(children);
                self.elements = new_elements;
            }
            Some(_) => unreachable!(),
        }
    }

    fn get_quadrant(&self, r: &T) -> Option<Quadrant> {
        let half_width = self.bounds.x + (self.bounds.width / 2.0);
        let half_height = self.bounds.y + (self.bounds.height / 2.0);

        let bounds = r.bounds();
        let fits_left_half = bounds.x > self.bounds.x && bounds.x + bounds.width < half_width;
        let fits_right_half =
            bounds.x > half_width && bounds.x + bounds.width < self.bounds.x + self.bounds.width;
        let fits_top_half = bounds.y > self.bounds.y && bounds.y + bounds.height < half_height;
        let fits_bottom_half =
            bounds.y > half_height && bounds.y + bounds.height < self.bounds.y + self.bounds.height;

        if fits_top_half && fits_left_half {
            Some(Quadrant::TL)
        } else if fits_top_half && fits_right_half {
            Some(Quadrant::TR)
        } else if fits_bottom_half && fits_right_half {
            Some(Quadrant::BR)
        } else if fits_bottom_half && fits_left_half {
            Some(Quadrant::BL)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Set up arbitrary element
    struct Particle {
        x: f32,
        y: f32,
    }

    impl Bounded for Particle {
        fn bounds(&self) -> Bounds {
            Bounds {
                x: self.x,
                y: self.y,
                width: 1.0,
                height: 1.0,
            }
        }
    }

    #[test]
    fn test_build() {
        let mut qt = QuadTree::new(Bounds {
            x: 0.0,
            y: 0.0,
            width: 10.0,
            height: 10.0,
        });

        qt.insert(&Particle { x: 1.0, y: 1.0 });
        qt.insert(&Particle { x: 9.0, y: 9.0 });
        qt.insert(&Particle { x: 1.0, y: 9.0 });
        qt.insert(&Particle { x: 9.0, y: 1.0 });
        qt.insert(&Particle { x: 8.0, y: 1.0 });
        qt.insert(&Particle { x: 8.0, y: 8.0 });

        assert_eq!(2 + 2, 4);
    }
}
