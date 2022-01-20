use druid::Lens;

pub struct SomeLens;

impl<T> Lens<Option<T>, T> for SomeLens {
    fn with<V, F: FnOnce(&T) -> V>(&self, data: &Option<T>, f: F) -> V {
        f(data.as_ref().unwrap())
    }

    fn with_mut<V, F: FnOnce(&mut T) -> V>(&self, data: &mut Option<T>, f: F) -> V {
        f(data.as_mut().unwrap())
    }
}
