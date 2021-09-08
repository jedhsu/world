
pub struct Computation {
    num_last_episodes: i32,


}

pub struct Computing {
    base: i32,
}

pub trait Algorithm: World {
    fn seed(&self);
    fn _get_observation(&self);
    fn step(&self);
    fn reset(&self);
}

impl<A> World for A where A: Algorithm {

}

impl Flow for {
    fn step(&self) {
    }
}
pub trait TapeAlgorithm: Algorithm {
}
