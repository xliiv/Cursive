/// Generic trait to enable chainable API
pub trait With: Sized {
    /// Calls the given closure on `self`.
    fn with<F: FnOnce(&mut Self)>(mut self, f: F) -> Self {
        f(&mut self);
        self
    }

    /// Calls the given closure on `self`.
    fn try_with<E, F>(mut self, f: F) -> Result<Self, E>
    where
        F: FnOnce(&mut Self) -> Result<(), E>,
    {
        f(&mut self)?;
        Ok(self)
    }
}

impl<T: Sized> With for T {}
