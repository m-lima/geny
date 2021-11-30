use super::Engine;
use crate::sim::Simulation;

pub struct Terminal<const BORDER: bool>;

impl<const BORDER: bool> Engine for Terminal<BORDER> {
    fn start(self, mut simulation: Simulation) {
        let mut day = 0_usize;
        while simulation.step() {
            println!("[67A");
            println!("[68ADay: [37m{}[m", day);
            render::<BORDER>(&simulation);
            day += 1;
        }
    }
}

// ALLOWED: Makes it easier to read
#[allow(clippy::non_ascii_literal)]
fn render<const BORDER: bool>(simulation: &Simulation) {
    if BORDER {
        print!("┏");
        for _ in 0..simulation.size() << 1 {
            print!("━");
        }
        println!("┓");
    }

    let mut buffer = vec![
        vec![Option::<(char, u32)>::None; simulation.size() as usize];
        simulation.size() as usize
    ];

    // ALLOWED: Coord is never negative or out of bounds, due to `World::advance`
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    for boop in simulation.boops() {
        let coord = boop.coordinate();
        let direction = {
            let dir = boop.direction().as_rad();
            if dir <= std::f32::consts::PI / 4. {
                '→'
            } else if dir <= std::f32::consts::PI * 3. / 4. {
                '↓'
            } else if dir <= std::f32::consts::PI * 5. / 4. {
                '←'
            } else if dir <= std::f32::consts::PI * 7. / 4. {
                '↑'
            } else {
                '→'
            }
        };
        let signature = boop.signature();
        buffer[coord.y() as usize][coord.x() as usize] = Some((direction, signature));
    }

    for row in buffer {
        if BORDER {
            print!("│");
        }

        for cell in row {
            if let Some((direction, mut signature)) = cell {
                let b = signature & 0xff;
                signature >>= 8;
                let g = signature & 0xff;
                signature >>= 8;
                let r = signature & 0xff;
                print!(
                    "[48;2;{};{};{}m[38;2;{};{};{}m{}[38;2;{};{};{}m\u{2588}[m",
                    r,
                    g,
                    b,
                    !r & 0xff,
                    !g & 0xff,
                    !b & 0xff,
                    direction,
                    r,
                    g,
                    b
                );
            } else {
                print!("  ");
            }
        }

        if BORDER {
            println!("│");
        } else {
            println!();
        }
    }

    if BORDER {
        print!("┗");
        for _ in 0..simulation.size() << 1 {
            print!("━");
        }
        print!("┛");
    }
    println!();
}
