
pub enum OptionalResult<T, E> {
    Ok(T),
    Err(E),
    None,
}
