use crate::{
    circle::Circle,
    gravity_solver::{GRAVITATIONAL_CONSTANT, GravitySolver},
    prelude::*,
    vec2::Vec2,
};

use wasm_bindgen::prelude::*;

mod animation_frame;
mod circle;
mod gravity_solver;
mod prelude;
mod vec2;

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    let document = document();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into().unwrap();

    let ctx: web_sys::CanvasRenderingContext2d = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into()
        .unwrap();

    let game_width = window().inner_width().unwrap().as_f64().unwrap();
    let game_height = window().inner_height().unwrap().as_f64().unwrap();
    let sun = Circle {
        pos: Vec2 {
            x: game_width as f32 / 2.0,
            y: game_height as f32 / 2.0,
        },
        velocity: Vec2 { x: 0.0, y: 0.0 },
        radius: 100.0,
        color: "yellow".to_string(),
    };

    let orbit_radius = 250.0;
    let orbital_speed = (GRAVITATIONAL_CONSTANT * sun.radius * sun.radius / orbit_radius).sqrt();

    let earth = Circle {
        pos: Vec2 {
            x: sun.pos.x + orbit_radius,
            y: sun.pos.y,
        },
        velocity: Vec2 {
            x: 0.0,
            y: orbital_speed,
        },
        radius: 20.0,
        color: "green".to_string(),
    };

    let mut objects = vec![sun, earth];

    let mut last_time: f64 = window().performance().unwrap().now();

    animation_frame::run(move |timestamp| {
        let dt = ((timestamp - last_time) / 1000.0) as f32;
        last_time = timestamp;
        let game_width = window().inner_width().unwrap().as_f64().unwrap();
        let game_height = window().inner_height().unwrap().as_f64().unwrap();

        let canvas_width = canvas.width();
        let canvas_height = canvas.height();

        if (canvas_width as f64) != game_width {
            canvas.set_width(game_width as u32);
        }

        if (canvas_height as f64) != game_height {
            canvas.set_height(game_height as u32);
        }

        ctx.clear_rect(0.0, 0.0, game_width, game_height);
        GravitySolver::apply(&mut objects, dt);
        for object in &objects {
            object.draw(&ctx);
        }
        true
    });
    Ok(())
}
