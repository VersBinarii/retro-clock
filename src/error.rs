use ufmt::derive::uDebug;

#[derive(uDebug)]
pub enum Error<E>{
    BmeSensor(E),
    Display,
}
