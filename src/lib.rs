use crate::{circle::Circle, gravity_solver::GravitySolver, prelude::*, vec2::Vec2};

use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::prelude::*;

mod animation_frame;
mod circle;
mod gravity_solver;
mod prelude;
mod vec2;

thread_local! {
    static OBJECTS: RefCell<Vec<Circle>> = const { RefCell::new(Vec::new()) };
    static PREVIEW: RefCell<Option<Circle>> = const { RefCell::new(None) };
    static PAN_OFFSET: RefCell<Vec2> = const { RefCell::new(Vec2 { x: 0.0, y: 0.0 }) };
}

struct PanState {
    dragging: bool,
    last_position: Vec2,
    offset: Vec2,
}

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

    let pan = Rc::new(RefCell::new(PanState {
        dragging: false,
        last_position: Vec2 { x: 0.0, y: 0.0 },
        offset: Vec2 { x: 0.0, y: 0.0 },
    }));

    let pan_on_mouse_down = Rc::clone(&pan);
    let on_mouse_down =
        Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |event: web_sys::MouseEvent| {
            if event.button() == 2 {
                event.prevent_default();
                let mut pan = pan_on_mouse_down.borrow_mut();
                pan.dragging = true;
                pan.last_position = Vec2 {
                    x: event.client_x() as f32,
                    y: event.client_y() as f32,
                };
            }
        });
    canvas.add_event_listener_with_callback("mousedown", on_mouse_down.as_ref().unchecked_ref())?;

    let pan_on_mouse_move = Rc::clone(&pan);
    let on_mouse_move =
        Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |event: web_sys::MouseEvent| {
            let mut pan = pan_on_mouse_move.borrow_mut();
            if pan.dragging {
                let position = Vec2 {
                    x: event.client_x() as f32,
                    y: event.client_y() as f32,
                };
                let delta = position - pan.last_position;
                pan.offset += delta;
                pan.last_position = position;
                PAN_OFFSET.with(|offset| *offset.borrow_mut() = pan.offset);
                event.prevent_default();
            }
        });
    window()
        .add_event_listener_with_callback("mousemove", on_mouse_move.as_ref().unchecked_ref())?;

    let pan_on_mouse_up = Rc::clone(&pan);
    let on_mouse_up =
        Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |event: web_sys::MouseEvent| {
            if event.button() == 2 {
                pan_on_mouse_up.borrow_mut().dragging = false;
            }
        });
    window().add_event_listener_with_callback("mouseup", on_mouse_up.as_ref().unchecked_ref())?;

    let on_context_menu =
        Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
            event.prevent_default();
        });
    canvas.add_event_listener_with_callback(
        "contextmenu",
        on_context_menu.as_ref().unchecked_ref(),
    )?;

    let mut last_time: f64 = window().performance().unwrap().now();

    animation_frame::run(move |timestamp| {
        let _ = (
            &on_mouse_down,
            &on_mouse_move,
            &on_mouse_up,
            &on_context_menu,
        );
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
        let offset = pan.borrow().offset;
        ctx.save();
        ctx.translate(offset.x as f64, offset.y as f64).unwrap();
        OBJECTS.with(|objects| {
            let mut objects = objects.borrow_mut();
            GravitySolver::apply(&mut objects, dt);
            for object in objects.iter() {
                object.draw(&ctx);
            }
        });
        PREVIEW.with(|preview| {
            if let Some(preview) = preview.borrow().as_ref() {
                OBJECTS.with(|objects| {
                    let path = GravitySolver::predict_path(preview, &objects.borrow());
                    preview.draw_preview(&ctx, &path);
                });
            }
        });
        ctx.restore();
        true
    });
    Ok(())
}

#[wasm_bindgen]
pub fn create_object(x: f32, y: f32, radius: f32, velocity_x: f32, velocity_y: f32, color: String) {
    OBJECTS.with(|objects| {
        objects.borrow_mut().push(Circle {
            pos: Vec2 { x, y },
            velocity: Vec2 {
                x: velocity_x,
                y: velocity_y,
            },
            radius,
            color,
        });
    });
}

#[wasm_bindgen]
pub fn set_preview(
    x: f32,
    y: f32,
    velocity_x: f32,
    velocity_y: f32,
    radius: f32,
    color: String,
    visible: bool,
) {
    if !visible {
        PREVIEW.with(|preview| *preview.borrow_mut() = None);
        return;
    }

    PREVIEW.with(|preview| {
        *preview.borrow_mut() = Some(Circle {
            pos: Vec2 { x, y },
            velocity: Vec2 {
                x: velocity_x,
                y: velocity_y,
            },
            radius,
            color,
        });
    });
}

#[wasm_bindgen]
pub fn get_pan_offset() -> Vec<f32> {
    PAN_OFFSET.with(|offset| {
        let offset = *offset.borrow();
        vec![offset.x, offset.y]
    })
}
