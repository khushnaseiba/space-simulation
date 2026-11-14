use crate::{circle::Circle, prelude::*, vec2::Vec2};

use wasm_bindgen::prelude::*;

mod animation_frame;
mod circle;
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
    let mut circle = Circle {
        pos: Vec2 {
            x: game_width as f32 / 2.0,
            y: game_height as f32 / 2.0,
        },
        velocity: Vec2 { x: 200.0, y: 400.0 },
        radius: 100.0,
    };

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
        circle.draw(&ctx);
        true
    });
    Ok(())
}
