mod qtree;

use cgmath::{Array, Point2, Vector2, Zero};
use rand::{thread_rng, Rng};
use rayon::prelude::*;

type Vec2 = Vector2<f32>;

const DT: f32 = 1e-7; // It has to be set to e-7 in order to match to the Taichi Result.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Particle {
    pub position: Point2<f32>,
    pub velocity: Vector2<f32>,
    pub mass: f32,
}

#[derive(Clone)]
pub struct Universe {
    time: usize,
    pub bodies: (Vec<Particle>, Vec<Particle>),
}

impl Universe {
    pub fn new(num_particles: usize) -> Self {
        let mut rng = thread_rng();

        let bodies0: Vec<Particle> = (0..num_particles)
            .map(|_| {
                let a = rng.gen::<f32>() * std::f32::consts::TAU;
                let r = rng.gen::<f32>().sqrt() * 0.3;

                let position = Point2 {
                    x: a.cos() * r + 0.5,
                    y: a.sin() * r + 0.5,
                };
                let velocity = Vector2::zero();
                let mass = rng.gen::<f32>() * 1.4 + 0.1;

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
}

fn gravity_func(distance: Vec2) -> Vec2 {
    // let l2 = distance.norm_sqr() + 1e-3;
    let l2 = distance.map(|x| x * 2.0f32).sum() + 1e-3f32;
    distance * (l2.powf((-3.0f32) / 2.0f32))
}

pub fn get_gravity_at_raw_seq(pos: Point2<f32>, bodies: &[Particle]) -> Vec2 {
    bodies
        .iter()
        .map(|p| gravity_func(p.position - pos) * p.mass)
        .sum()
}

pub fn get_gravity_at_raw_par(pos: Point2<f32>, bodies: &[Particle]) -> Vec2 {
    bodies
        .par_iter()
        .map(|p| gravity_func(p.position - pos) * p.mass)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::Universe;

    #[test]
    fn it_works() {
        let mut universe = Universe::new(1024);
        universe.next_state_par();

        assert_eq!(2 + 2, 4);
    }
}
