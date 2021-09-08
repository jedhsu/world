pub enum Schedule {
    Cyclic,
}

pub trait Episode<S, T>
where
    S: Schedule,
{
    fn get_action(&self) -> 
}
