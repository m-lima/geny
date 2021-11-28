use super::super::being::Being;
use super::super::geo::Direction;
use super::{Renderer, World};
pub struct Terminal<const BORDER: bool>;

impl<const BORDER: bool> Renderer for Terminal<BORDER> {
    // ALLOWED: Makes it easier to read
    #[allow(clippy::non_ascii_literal)]
    fn render(&self, world: &World) {
        if BORDER {
            print!("┏");
            for _ in 0..world.size() << 1 {
                print!("━");
            }
            println!("┓");
        }
        let mut buffer =
            vec![vec![Option::<&Being>::None; world.size() as usize]; world.size() as usize];

        for (being, coord) in world
            .coordinates()
            .enumerate()
            .map(|(i, c)| (world.being(i), c))
        {
            buffer[coord.y() as usize][coord.x() as usize] = Some(being);
        }

        for row in buffer {
            if BORDER {
                print!("│");
            }
            for cell in row {
                if let Some(being) = cell {
                    let mut color = being.as_u24();
                    let b = color & 0xff;
                    color >>= 8;
                    let g = color & 0xff;
                    color >>= 8;
                    let r = color & 0xff;
                    let direction = match being.direction() {
                        Direction::North => "↑",
                        Direction::East => "→",
                        Direction::South => "↓",
                        Direction::West => "←",
                    };
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
            for _ in 0..world.size() << 1 {
                print!("━");
            }
            print!("┛");
        }
        println!();
    }
}
