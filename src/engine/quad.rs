use super::Engine;
use crate::sim::{Direction, Simulation};

pub struct Quad(macroquad::window::Conf);

impl Quad {
    pub fn new(conf: macroquad::window::Conf) -> Self {
        Self(conf)
    }
}

impl Engine for Quad {
    fn start(self, mut simulation: Simulation) {
        macroquad::Window::from_config(self.0, async move {
            let mut day = 0_usize;
            while simulation.step() {
                macroquad::window::clear_background(macroquad::color::Color::from_rgba(
                    33, 33, 33, 255,
                ));

                let x_scale =
                    (macroquad::window::screen_width() - 10.) / f32::from(simulation.size());
                let y_scale =
                    (macroquad::window::screen_height() - 10.) / f32::from(simulation.size());

                for boop in simulation.boops() {
                    let coord = boop.coordinate();
                    let direction = match boop.direction() {
                        Direction::North => -90.,
                        Direction::East => 0.,
                        Direction::South => 90.,
                        Direction::West => 180.,
                    };
                    let color = signature_to_color(boop.signature());

                    macroquad::shapes::draw_poly(
                        10. + f32::from(coord.x()) * x_scale,
                        10. + f32::from(coord.y()) * y_scale,
                        3,
                        10.,
                        direction,
                        color,
                    );
                }
                macroquad::text::draw_text(
                    &format!("Day: {}", day),
                    0.,
                    0.,
                    12.,
                    macroquad::color::WHITE,
                );

                day += 1;
                macroquad::window::next_frame().await;
            }
        });
    }
}

fn signature_to_color(mut signature: u32) -> macroquad::color::Color {
    let b = signature & 0xff;
    signature >>= 8;
    let g = signature & 0xff;
    signature >>= 8;
    let r = signature & 0xff;

    // ALLOWED: The above bitmasks clamp the value already
    #[allow(clippy::cast_possible_truncation)]
    macroquad::color::Color::from_rgba(r as u8, g as u8, b as u8, 0xff)
}
