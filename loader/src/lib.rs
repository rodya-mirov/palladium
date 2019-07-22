use quicksilver::prelude::*;

type QsResult<T> = quicksilver::prelude::Result<T>;

pub trait Loadable {
    fn is_loaded(&mut self) -> QsResult<bool>;
}

impl<T> Loadable for Asset<T> {
    fn is_loaded(&mut self) -> QsResult<bool> {
        let mut loaded = false;

        self.execute(|_| {
            loaded = true;
            Ok(())
        })?;

        Ok(loaded)
    }
}

impl<T> Loadable for &mut T
where
    T: Loadable,
{
    fn is_loaded(&mut self) -> QsResult<bool> {
        (*self).is_loaded()
    }
}

impl<T> Loadable for Option<T>
where
    T: Loadable,
{
    fn is_loaded(&mut self) -> QsResult<bool> {
        match self.as_mut() {
            Some(loadable) => loadable.is_loaded(),
            None => Ok(true),
        }
    }
}

impl<T> Loadable for Vec<T>
where
    T: Loadable,
{
    fn is_loaded(&mut self) -> QsResult<bool> {
        let mut loaded = true;
        for loadable in self.iter_mut() {
            loaded &= loadable.is_loaded()?;
        }
        Ok(loaded)
    }
}
