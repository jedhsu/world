/// An observation is a `Realized`.
pub struct Realized<T> {
    datum: Datum<T>,
    step: Time,
}

pub trait Realizing {
    fn next_datum(&self) -> Datum;
    fn flow(&self) -> Flow;
}
