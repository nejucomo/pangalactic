#[derive(Debug, Copy, Clone)]
pub struct BindRef<'a, R, V>
where
    R: ?Sized,
{
    pub bound: &'a R,
    pub value: V,
}

#[derive(Debug)]
pub struct BindRefMut<'a, R, V>
where
    R: ?Sized,
{
    pub bound: &'a mut R,
    pub value: V,
}

pub trait Bindable {
    fn bind_ref<V>(&self, value: V) -> BindRef<Self, V> {
        BindRef { bound: self, value }
    }

    fn bind_ref_mut<V>(&mut self, value: V) -> BindRefMut<Self, V> {
        BindRefMut { bound: self, value }
    }
}
