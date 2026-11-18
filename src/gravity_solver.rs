use crate::{circle::Circle, vec2::Vec2};

pub const GRAVITATIONAL_CONSTANT: f32 = 500.0;

pub struct GravitySolver;

impl GravitySolver {
    pub fn predict_path(object: &Circle, objects: &[Circle]) -> Vec<Vec2> {
        let mut prediction = objects.to_vec();
        prediction.push(object.clone());
        let mut path = Vec::with_capacity(160);

        for _ in 0..160 {
            Self::apply(&mut prediction, 0.05);
            path.push(prediction.last().unwrap().pos);
        }

        path
    }

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
