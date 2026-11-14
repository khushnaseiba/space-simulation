use crate::{circle::Circle, vec2::Vec2};

pub const GRAVITATIONAL_CONSTANT: f32 = 500.0;

pub struct GravitySolver;

impl GravitySolver {
    pub fn apply(objects: &mut [Circle], dt: f32) {
        let mut accelerations = vec![Vec2 { x: 0.0, y: 0.0 }; objects.len()];

        for (i, acceleration) in accelerations.iter_mut().enumerate() {
            for (j, attractor) in objects.iter().enumerate() {
                if i == j {
                    continue;
                }

                let to_attractor = attractor.pos - objects[i].pos;
                let distance_squared = to_attractor.length_squared();

                if distance_squared == 0.0 {
                    continue;
                }

                let distance = distance_squared.sqrt();
                let mass = attractor.radius * attractor.radius;

                *acceleration +=
                    to_attractor * (GRAVITATIONAL_CONSTANT * mass / (distance_squared * distance));
            }
        }

        for (object, acceleration) in objects.iter_mut().zip(accelerations) {
            object.velocity += acceleration * dt;
            object.pos += object.velocity * dt;
        }
    }
}
