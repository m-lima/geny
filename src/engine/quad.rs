use super::Engine;
use crate::sim::Simulation;

pub struct Quad(macroquad::window::Conf);

impl Quad {
    pub fn new(mut conf: macroquad::window::Conf) -> Self {
        conf.window_height += 20;
        Self(conf)
    }
}

impl Engine for Quad {
    fn start(self, mut simulation: Simulation, days: usize) {
        macroquad::Window::from_config(self.0, async move {
            let mut gen = 0_usize;
            loop {
                for day in 0..days {
                    if macroquad::input::is_key_down(macroquad::input::KeyCode::Escape) {
                        return;
                    }

                    simulation.step();
                    render(&simulation, gen, day);
                    macroquad::window::next_frame().await;
                }

                gen += 1;
                // if !simulation.next_generation() {
                //     break;
                // }
            }
        });
    }
}

fn render(simulation: &Simulation, gen: usize, day: usize) {
    macroquad::window::clear_background(macroquad::color::Color::from_rgba(33, 33, 33, 255));

    let width = macroquad::window::screen_width();
    let scale = width / f32::from(simulation.size());
    let scale2 = scale / 2.;

    macroquad::shapes::draw_rectangle(
        0.,
        0.,
        width,
        20.,
        macroquad::color::Color::from_rgba(22, 22, 22, 255),
    );
    macroquad::text::draw_text(
        &format!("Generation: {gen} Day: {day}"),
        2.,
        14.,
        24.,
        macroquad::color::WHITE,
    );

    for boop in simulation.boops() {
        let coord = boop.coordinate();
        let direction = boop.direction().as_rad().to_degrees();
        let color = signature_to_color(boop.signature());

        macroquad::shapes::draw_poly(
            scale2 + coord.x() * scale,
            20. + scale2 + coord.y() * scale,
            3,
            scale2,
            direction,
            color,
        );
    }

    for food in simulation.fodder() {
        macroquad::shapes::draw_circle(
            scale2 + food.x() * scale,
            20. + scale2 + food.y() * scale,
            scale2,
            macroquad::color::GREEN,
        );
    }
}

fn signature_to_color(mut signature: u32) -> macroquad::color::Color {
    let b = truncate!(u32 -> u8, signature & 0xff);
    signature >>= 8;
    let g = truncate!(u32 -> u8, signature & 0xff);
    signature >>= 8;
    let r = truncate!(u32 -> u8, signature & 0xff);

    macroquad::color::Color::from_rgba(r, g, b, 0xff)
}
