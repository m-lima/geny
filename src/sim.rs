use super::being::Being;
use super::neural::Brain;
use super::world::World;

struct Simulation {
    world: World,
    beigns: Vec<Being>,
    brains: Vec<Brain>,
}
