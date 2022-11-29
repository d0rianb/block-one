use std::ops::Mul;
use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::Graphics2D;
use speedy2d::shape::{Polygon, Rectangle};

#[inline]
pub fn draw_rounded_rectangle(x: f32, y: f32, width: f32, height: f32, radius: f32, color: Color, graphics: &mut Graphics2D) {
    graphics.draw_circle(Vector2::new(x + radius, y + radius), radius, color);
    graphics.draw_circle(Vector2::new(x + width - radius, y + radius), radius, color);
    graphics.draw_circle(Vector2::new(x + radius, y + height - radius), radius, color);
    graphics.draw_circle(Vector2::new(x + width - radius, y + height - radius), radius, color);
    graphics.draw_rectangle(Rectangle::new(Vector2::new(x + radius, y), Vector2::new(x + width - radius, y + height)), color);
    graphics.draw_rectangle(Rectangle::new(Vector2::new(x, y + radius), Vector2::new(x + width, y + height - radius)), color);
}

#[inline]
pub fn draw_rectangle(x: f32, y: f32, width: f32, height: f32, color: Color, graphics: &mut Graphics2D) {
    graphics.draw_rectangle(Rectangle::new(Vector2::new(x, y), Vector2::new(x + width, y + height )), color);
}

#[inline]
pub fn _draw_rect_border(origin: Vector2<f32>, width: f32, height: f32, thickness: f32, border_color: Color, graphics: &mut Graphics2D) {
    graphics.draw_line(origin, origin + Vector2::new(width, 0.), thickness, border_color);
    graphics.draw_line(origin + Vector2::new(width, 0.), origin + Vector2::new(width, height), thickness, border_color);
    graphics.draw_line(origin + Vector2::new(width, height), origin + Vector2::new(0., height), thickness, border_color);
    graphics.draw_line(origin + Vector2::new(0., height), origin, thickness, border_color);
}


#[inline]
pub fn draw_rounded_rectangle_with_border(x: f32, y: f32, width: f32, height: f32, radius: f32, border_width: f32, bg_color: Color, border_color: Color, graphics: &mut Graphics2D) {
    // draw border
    draw_rounded_rectangle(x - border_width, y - border_width, width + 2. * border_width, height + 2. * border_width, radius - border_width, border_color, graphics);
    // draw background
    draw_rounded_rectangle(x, y, width, height, radius, bg_color, graphics);
}

#[inline]
pub fn draw_rounded_line(x: f32, y: f32, width: f32, height: f32, color: Color, graphics: &mut Graphics2D) {
    let radius= width / 2.;
    graphics.draw_circle(Vector2::new(x + radius, y + radius), radius, color);
    graphics.draw_circle(Vector2::new(x + radius, y + height - radius), radius, color);
    graphics.draw_rectangle(Rectangle::new(Vector2::new(x + radius, y), Vector2::new(x + width - radius, y + height)), color);
    graphics.draw_rectangle(Rectangle::new(Vector2::new(x, y + radius), Vector2::new(x + width, y + height - radius)), color);
}

type Point = Vector2<f32>;

#[inline]
pub fn draw_bezier_curve(start: Point, control1: Point, control2: Point, end: Point, graphics: &mut Graphics2D) {
    let nb_subdivision = 100;
    let mut points = vec![start];
    // DEBUG
    graphics.draw_circle(start, 2., Color::RED);
    graphics.draw_circle(control1, 2., Color::RED);
    graphics.draw_circle(control2, 2., Color::RED);
    graphics.draw_circle(end, 2., Color::RED);
    for i in 0 .. nb_subdivision {
        let t = (i as f32 + 1.) / nb_subdivision as f32;
        assert!(0. <= t && t <= 1.);
        let new_point = start.mul((1.-t).powf(3.)) + control1.mul(3.*(1.-t).powf(2.)*t) + control2.mul(3.*(1.-t)*t.powf(2.)) + end.mul(t.powf(3.)); // Bezier polynom
        if i >= 1 { graphics.draw_line(points[i-1], points[i], 1., Color::BLACK); } // draw the curve
        points.push(new_point);
    }
    graphics.draw_line(points.last().unwrap(), end, 1., Color::BLACK);
}