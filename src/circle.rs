use std::f64::consts::PI;

use web_sys::CanvasRenderingContext2d;

use crate::vec2::Vec2;

#[derive(Clone)]
pub struct Circle {
    pub pos: Vec2,
    pub velocity: Vec2,
    pub radius: f32,
    pub color: String,
}

impl Circle {
    pub fn draw(&self, ctx: &CanvasRenderingContext2d) {
        ctx.set_fill_style_str(&self.color);
        ctx.begin_path();
        ctx.arc(
            self.pos.x as f64,
            self.pos.y as f64,
            self.radius as f64,
            0.0,
            PI * 2.0,
        )
        .unwrap();
        ctx.fill();
    }

    pub fn draw_preview(&self, ctx: &CanvasRenderingContext2d, path: &[Vec2]) {
        ctx.save();
        ctx.set_global_alpha(0.45);
        ctx.set_stroke_style_str(&self.color);
        ctx.set_line_width(1.5);
        ctx.begin_path();
        ctx.move_to(self.pos.x as f64, self.pos.y as f64);
        for point in path {
            ctx.line_to(point.x as f64, point.y as f64);
        }
        ctx.stroke();

        ctx.set_global_alpha(0.35);
        ctx.set_fill_style_str(&self.color);
        ctx.begin_path();
        ctx.arc(
            self.pos.x as f64,
            self.pos.y as f64,
            self.radius as f64,
            0.0,
            PI * 2.0,
        )
        .unwrap();
        ctx.fill();

        let tip_x = self.pos.x + self.velocity.x;
        let tip_y = self.pos.y + self.velocity.y;
        let angle = self.velocity.y.atan2(self.velocity.x);
        let arrowhead_length = 12.0;

        ctx.set_global_alpha(1.0);
        ctx.set_line_width(2.0);
        ctx.begin_path();
        ctx.move_to(self.pos.x as f64, self.pos.y as f64);
        ctx.line_to(tip_x as f64, tip_y as f64);
        if self.velocity.length_squared() > 0.0 {
            ctx.move_to(tip_x as f64, tip_y as f64);
            ctx.line_to(
                (tip_x - arrowhead_length * (angle - PI as f32 / 6.0).cos()) as f64,
                (tip_y - arrowhead_length * (angle - PI as f32 / 6.0).sin()) as f64,
            );
            ctx.move_to(tip_x as f64, tip_y as f64);
            ctx.line_to(
                (tip_x - arrowhead_length * (angle + PI as f32 / 6.0).cos()) as f64,
                (tip_y - arrowhead_length * (angle + PI as f32 / 6.0).sin()) as f64,
            );
        }
        ctx.stroke();
        ctx.restore();
    }
}
