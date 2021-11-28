use super::super::geo::Direction;
use super::super::sim::Simulation;
use super::Renderer;

pub struct Terminal<const BORDER: bool>;

impl<const BORDER: bool> Renderer for Terminal<BORDER> {
    // ALLOWED: Makes it easier to read
    #[allow(clippy::non_ascii_literal)]
    fn render(&self, simulation: &Simulation) {
        let world = simulation.world();

        if BORDER {
            print!("┏");
            for _ in 0..world.size() << 1 {
                print!("━");
            }
            println!("┓");
        }
        let mut buffer =
            vec![vec![Option::<(char, u32)>::None; world.size() as usize]; world.size() as usize];

        for index in simulation.indices() {
            let coord = world.being(index);
            let direction = match simulation.being(index).direction() {
                Direction::North => '↑',
                Direction::East => '→',
                Direction::South => '↓',
                Direction::West => '←',
            };
            let id = simulation.genome(index).id();
            buffer[coord.y() as usize][coord.x() as usize] = Some((direction, id));
        }

        for row in buffer {
            if BORDER {
                print!("│");
            }
            for cell in row {
                if let Some((direction, mut id)) = cell {
                    let b = id & 0xff;
                    id >>= 8;
                    let g = id & 0xff;
                    id >>= 8;
                    let r = id & 0xff;
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
