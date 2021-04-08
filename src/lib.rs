pub mod vec2;

use crate::vec2::Vec2;
use rand::{thread_rng, Rng};
use rayon::prelude::*;

const DT: f64 = 1e-7; // It has to be set to e-7 in order to match to the Taichi Result.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Particle {
    pub position: Vec2,
    pub velocity: Vec2,
    pub mass: f64,
}

// pub struct Universe(pub Vec<Particle>);
//
// impl Universe {
//     pub fn new(num_particles: usize) -> Self {
//         let mut rng = thread_rng();
//         let bodies0: Vec<Particle> = (0..num_particles)
//             .map(|_| {
//                 let a = rng.gen::<f64>() * std::f64::consts::TAU;
//                 let r = rng.gen::<f64>().sqrt() * 0.3;
//
//                 let position = Vec2 {
//                     x: a.cos() * r + 0.5,
//                     y: a.sin() * r + 0.5,
//                 };
//                 let velocity = Vec2::zeros();
//                 let mass = rng.gen::<f64>() * 1.4 + 0.1;
//
//                 Particle {
//                     position,
//                     velocity,
//                     mass,
//                 }
//             })
//             .collect();
//         Self { 0: bodies0 }
//     }
//
//     pub fn next_state_seq(&self) -> Universe {
//         let in_bodies = &self.0;
//
//         let next_state = in_bodies
//             .iter()
//             .map(|curr: &Particle| {
//                 let acc = get_raw_gravity_at(curr.position, in_bodies);
//
//                 let velocity = curr.velocity + acc * DT;
//                 let position = curr.position + velocity;
//
//                 Particle {
//                     position,
//                     velocity,
//                     mass: curr.mass,
//                 }
//             })
//             .collect();
//
//         Universe { 0: next_state }
//     }
//
//     pub fn next_state_par(&self) -> Universe {
//         let in_bodies = &self.0;
//
//         let next_state = in_bodies
//             .par_iter()
//             .map(|curr: &Particle| {
//                 let acc = get_raw_gravity_at(curr.position, in_bodies);
//
//                 let velocity = curr.velocity + acc * DT;
//                 let position = curr.position + velocity;
//
//                 Particle {
//                     position,
//                     velocity,
//                     mass: curr.mass,
//                 }
//             })
//             .collect();
//
//         Universe { 0: next_state }
//     }
// }

#[derive(Clone)]
pub struct Universe {
    time: usize,
    bodies: (Vec<Particle>, Vec<Particle>),
}

impl Universe {
    pub fn new(num_particles: usize) -> Self {
        let mut rng = thread_rng();

        let bodies0: Vec<Particle> = (0..num_particles)
            .map(|_| {
                let a = rng.gen::<f64>() * std::f64::consts::TAU;
                let r = rng.gen::<f64>().sqrt() * 0.3;

                let position = Vec2 {
                    x: a.cos() * r + 0.5,
                    y: a.sin() * r + 0.5,
                };
                let velocity = Vec2::zeros();
                let mass = rng.gen::<f64>() * 1.4 + 0.1;

                Particle {
                    position,
                    velocity,
                    mass,
                }
            })
            .collect();

        let bodies1 = bodies0.clone();

        Universe {
            time: 0,
            bodies: (bodies0, bodies1),
        }
    }

    pub fn next_state_seq(&mut self) -> &[Particle] {
        let (in_bodies, out_bodies) = if (self.time & 1) == 0 {
            (&self.bodies.0, &mut self.bodies.1)
        } else {
            (&self.bodies.1, &mut self.bodies.0)
        };

        for (out, prev) in out_bodies.iter_mut().zip(&in_bodies[..]) {
            let acc = get_gravity_at_raw_seq(prev.position, in_bodies);

            out.velocity += acc * DT;
            out.position = prev.position + out.velocity;
        }

        self.time += 1;

        out_bodies
    }

    pub fn next_state_par(&mut self) -> &[Particle] {
        let (in_bodies, out_bodies) = if (self.time & 1) == 0 {
            (&self.bodies.0, &mut self.bodies.1)
        } else {
            (&self.bodies.1, &mut self.bodies.0)
        };

        out_bodies
            .par_iter_mut()
            .zip(&in_bodies[..])
            .for_each(|(out, prev)| {
                let acc = get_gravity_at_raw_par(prev.position, in_bodies);

                out.velocity += acc * DT;
                out.position = prev.position + out.velocity;
            });

        self.time += 1;

        out_bodies
    }

    pub fn get_next_state_par(&self) {}
}

fn gravity_func(distance: Vec2) -> Vec2 {
    let l2 = distance.norm_sqr() + 1e-3;
    distance * (l2.powf((-3.0) / 2.0))
}

fn get_gravity_at_raw_seq(pos: Vec2, bodies: &[Particle]) -> Vec2 {
    bodies
        .par_iter()
        .map(|p| gravity_func(p.position - pos) * p.mass)
        .sum()
}

fn get_gravity_at_raw_par(pos: Vec2, bodies: &[Particle]) -> Vec2 {
    bodies
        .par_iter()
        .map(|p| gravity_func(p.position - pos) * p.mass)
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
