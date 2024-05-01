use super::Engine;
use crate::sim::Simulation;

pub struct Terminal<const BORDER: bool, const CLEAR: bool>;

impl<const BORDER: bool, const CLEAR: bool> Engine for Terminal<BORDER, CLEAR> {
    fn start(self, mut simulation: Simulation, days: usize) {
        let mut gen = 0_usize;

        let mut buffer = vec![vec![None; simulation.size() as usize]; simulation.size() as usize];

        loop {
            for day in 0..days {
                simulation.step();
                render::<BORDER, CLEAR>(&simulation, gen, day, &mut buffer);
            }
            gen += 1;
            if !simulation.next_generation() {
                break;
            }
        }
    }
}

fn render<const BORDER: bool, const CLEAR: bool>(
    simulation: &Simulation,
    gen: usize,
    day: usize,
    buffer: &mut Vec<Vec<Option<(char, u32)>>>,
) {
    use std::io::Write;

    let mut stdout = std::io::stdout().lock();

    if CLEAR {
        let height = simulation.size() + 1 + if BORDER { 2 } else { 0 };
        let _ = writeln!(stdout, "[{height}AGeneration: [37m{gen}[m Day: [37m{day}[m");
    } else {
        let _ = writeln!(stdout, "Generation: [37m{gen}[m Day: [37m{day}[m");
    }

    if BORDER {
        let _ = write!(stdout, "┏");
        for _ in 0..simulation.size() << 1 {
            let _ = write!(stdout, "━");
        }
        let _ = writeln!(stdout, "┓");
    }

    buffer.iter_mut().flatten().for_each(|c| *c = None);

    for boop in simulation.boops() {
        let coord = boop.coordinate();
        let direction = {
            let dir = boop.direction().as_rad();
            if dir <= std::f32::consts::FRAC_PI_4 {
                '→'
            } else if dir <= std::f32::consts::FRAC_PI_4 * 3. {
                '↓'
            } else if dir <= std::f32::consts::FRAC_PI_4 * 5. {
                '←'
            } else if dir <= std::f32::consts::FRAC_PI_4 * 7. {
                '↑'
            } else {
                '→'
            }
        };
        let signature = boop.signature();

        // SAFETY: Coordinate is always in the 0..size range
        unsafe {
            *buffer
                .get_unchecked_mut(usize::from(coord.y_index()))
                .get_unchecked_mut(usize::from(coord.x_index())) = Some((direction, signature));
        }
    }

    for row in buffer {
        if BORDER {
            print!("┃");
        }

        for cell in row {
            if let Some((direction, mut signature)) = cell {
                let b = signature & 0xff;
                signature >>= 8;
                let g = signature & 0xff;
                signature >>= 8;
                let r = signature & 0xff;
                print!(
                    "[48;2;{r};{g};{b}m[38;2;{};{};{}m{direction}[38;2;{r};{g};{b}m\u{2588}[m",
                    !r & 0xff,
                    !g & 0xff,
                    !b & 0xff,
                );
            } else {
                print!("  ");
            }
        }

        if BORDER {
            let _ = writeln!(stdout, "┃");
        } else {
            let _ = writeln!(stdout);
        }
    }

    if BORDER {
        let _ = write!(stdout, "┗");
        for _ in 0..simulation.size() << 1 {
            let _ = write!(stdout, "━");
        }
        let _ = write!(stdout, "┛");
    }
    let _ = writeln!(stdout);
}
