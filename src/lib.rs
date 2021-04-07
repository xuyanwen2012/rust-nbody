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

#[derive(Clone)]
pub struct ParticleList {
    time: usize,
    bodies: (Vec<Particle>, Vec<Particle>),
}

impl ParticleList {
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

        ParticleList {
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
            let acc = get_raw_gravity_at(prev.position, in_bodies);

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
                let acc = get_raw_gravity_at(prev.position, in_bodies);

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

fn get_raw_gravity_at(pos: Vec2, bodies: &[Particle]) -> Vec2 {
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
