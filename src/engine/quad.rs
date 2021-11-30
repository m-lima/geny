use super::Engine;
use crate::sim::Simulation;

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

                let scale = macroquad::window::screen_height() / f32::from(simulation.size());
                let scale2 = scale / 2.;

                for boop in simulation.boops() {
                    let coord = boop.coordinate();
                    let direction = boop.direction().as_rad().to_degrees();
                    let color = signature_to_color(boop.signature());

                    macroquad::shapes::draw_poly(
                        scale2 + coord.x() * scale,
                        scale2 + coord.y() * scale,
                        3,
                        scale2,
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
