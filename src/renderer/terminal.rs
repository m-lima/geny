use super::super::being::Being;
use super::{Renderer, World};
pub struct Terminal;

impl Renderer for Terminal {
    fn render(&self, world: &World) {
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
            for cell in row {
                if let Some(being) = cell {
                    let mut color = being.as_u24();
                    let b = color & 0xff;
                    color >>= 8;
                    let g = color & 0xff;
                    color >>= 8;
                    let r = color & 0xff;
                    print!("[38;2;{:03};{:03};{:03}m\u{2588}\u{2588}[m", r, g, b);
                } else {
                    print!("  ");
                }
            }
            println!();
        }
        println!();
    }
}
