
pub trait IntoResult: Sized {
    #[inline(always)]
    fn into_ok<E>(self) -> Result<Self, E> {
        Ok(self)
    }
    
    #[inline(always)]
    fn into_err<O>(self) -> Result<O, Self> {
        Err(self)
    }
}

impl<T> IntoResult for T {}
