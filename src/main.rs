mod renderer {
    use super::world::World;

    pub trait Renderer {
        fn render(world: &(impl 'static + World));
    }

    mod terminal {
        use super::{Renderer, World};
        pub struct Terminal;

        impl Renderer for Terminal {
            fn render(world: &(impl 'static + World)) {
                let dimensions = world.dimensions();

                if dimensions.len() > 2 {
                    panic!(
                        "Can only have a maximum of two dimension. Got {}",
                        dimensions.len()
                    );
                }

                let mut dimensions = dimensions.iter();
                if let Some(dimension) = dimensions.next() {
                    print!("{}", world.size(*dimension));
                }
                for dimention in dimensions {
                    print!(" {}", world.size(*dimention));
                }
                println!();
            }
        }
    }

    pub use terminal::Terminal;
}

mod world {
    pub trait World {
        type Dimension: Copy + Clone + Eq + PartialEq + Ord + PartialOrd;

        fn dimensions(&self) -> &'static [Self::Dimension];
        fn size(&self, dimension: Self::Dimension) -> usize;
    }

    mod grid {
        use super::World;

        pub struct Grid(usize, usize);
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Dimension(bool);

        impl Grid {
            pub fn new(width: usize, height: usize) -> Self {
                Self(width, height)
            }
        }

        impl World for Grid {
            type Dimension = Dimension;

            fn dimensions(&self) -> &'static [Self::Dimension] {
                &[Dimension(false), Dimension(true)]
            }

            fn size(&self, dimension: Self::Dimension) -> usize {
                if dimension.0 {
                    self.1
                } else {
                    self.0
                }
            }
        }
    }

    pub use grid::Grid;
}

mod brain {}

mod gene {}

fn main() -> anyhow::Result<()> {
    use std::io::Write;

    let mut stdout = std::io::stdout();
    let stdin = std::io::stdin();

    stdout.write_all(b"Select grid width: ")?;
    stdout.flush()?;

    let mut buffer = String::new();
    stdin.read_line(&mut buffer)?;
    let width = buffer.trim().parse::<usize>()?;

    stdout.write_all(b"Select grid height: ")?;
    stdout.flush()?;

    let mut buffer = String::new();
    stdin.read_line(&mut buffer)?;
    let height = buffer.trim().parse::<usize>()?;

    let grid = world::Grid::new(width, height);
    <renderer::Terminal as renderer::Renderer>::render(&grid);

    Ok(())
}
