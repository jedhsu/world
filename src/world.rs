pub trait World {
    pub type world: Realizing;

    fn step(&self, action: &Action) -> Result<i32, i32>;
    fn reset(&self) -> Result<i32, i32>;
    fn render(&self) -> Result;
    fn close(&self) -> Result;
}
