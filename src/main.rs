mod renderer {
    use super::world::World;

    pub trait Renderer {
        fn render(&self, world: &World);
    }

    // TODO: Put behind feature for 256 color term
    mod terminal {
        use super::{Renderer, World};
        pub struct Terminal;

        impl Renderer for Terminal {
            fn render(&self, world: &World) {
                for row in world.iter_rows() {
                    for cell in row {
                        if let Some(being) = cell {
                            let mut color = being.as_u24();
                            let b = color & 0xff;
                            color >>= 8;
                            let g = color & 0xff;
                            color >>= 8;
                            let r = color & 0xff;
                            print!("[38;2;{};{};{}m\u{2588}\u{2588}[m", r, g, b);
                        } else {
                            print!("  ");
                        }
                    }
                    println!();
                }
            }
        }
    }

    pub use terminal::Terminal;
}

mod world {
    use super::being::Being;

    pub struct World {
        size: usize,
        grid: [Option<Being>; 256 * 256],
    }

    impl World {
        #[inline]
        pub fn new(size: u8) -> Self {
            Self {
                size: size as usize,
                grid: [0; 256 * 256].map(|_| None),
            }
        }

        #[inline]
        pub const fn size(&self) -> usize {
            self.size
        }

        pub fn iter_rows(&self) -> impl Iterator<Item = iter::Row<'_>> {
            (0..self.size).map(|row| iter::Row::new(self, row))
        }

        pub fn step(&mut self) {
            let all_info = 0;
            self.grid
                .iter()
                .take(self.size * self.size)
                .filter_map(Option::as_ref)
                .for_each(|being| being.step(all_info));
        }

        // TODO: Should this be part of the builder?
        pub fn randomize(&mut self, count: usize) {
            use rand::seq::SliceRandom;

            // TODO: This assert is kinda ugly
            assert!(count < self.size * self.size);

            self.grid
                .iter_mut()
                .take(count)
                .for_each(|cell| *cell = Some(Being));
            self.grid[..self.size * self.size].shuffle(&mut rand::thread_rng());
        }
    }

    mod iter {
        use super::{Being, World};

        pub struct Row<'a> {
            first: usize,
            current: usize,
            world: &'a World,
        }

        impl<'a> Row<'a> {
            pub fn new(world: &'a World, row: usize) -> Self {
                Self {
                    first: row * world.size,
                    current: 0,
                    world,
                }
            }
        }

        impl<'a> Iterator for Row<'a> {
            type Item = &'a Option<Being>;

            fn next(&mut self) -> Option<Self::Item> {
                if self.current < self.world.size {
                    // SAFETY: Already check for bounds
                    let next = unsafe { self.world.grid.get_unchecked(self.first + self.current) };
                    self.current += 1;
                    Some(next)
                } else {
                    None
                }
            }
        }
    }
}

mod brain {
    enum Input {
        Age,
    }
    enum Output {
        MoveUp,
        MoveRight,
        MoveDown,
        MoveLeft,
    }

    struct Brain {}
}

mod being {
    pub struct Being;

    impl Being {
        pub fn as_u24(&self) -> u32 {
            rand::random()
        }

        pub fn step(&self, _all_info_needed: usize) {}
    }
}

fn main() -> anyhow::Result<()> {
    use std::io::Write;

    let mut stdout = std::io::stdout();
    let stdin = std::io::stdin();

    stdout.write_all(b"Select grid size: ")?;
    stdout.flush()?;

    let mut buffer = String::new();
    stdin.read_line(&mut buffer)?;
    let size = buffer.trim().parse::<u8>()?;

    let mut grid = world::World::new(size);
    grid.randomize(8);

    renderer::Renderer::render(&renderer::Terminal, &grid);

    Ok(())
}
