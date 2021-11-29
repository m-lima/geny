use super::Renderer;
use crate::sim::{Direction, Simulation};

pub struct Terminal<const BORDER: bool>;

impl<const BORDER: bool> Renderer for Terminal<BORDER> {
    // ALLOWED: Makes it easier to read
    #[allow(clippy::non_ascii_literal)]
    fn render(&self, simulation: &Simulation) {
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

        for boop in simulation.boops() {
            let coord = boop.coordinate();
            let direction = match boop.direction() {
                Direction::North => '↑',
                Direction::East => '→',
                Direction::South => '↓',
                Direction::West => '←',
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
}
