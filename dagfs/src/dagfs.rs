use pangalactic_dagio::Dagio;
use pangalactic_store::Store;

#[derive(Debug, Default, derive_more::Deref, derive_more::DerefMut)]
pub struct Dagfs<S>(Dagio<S>)
where
    S: Store;

impl<S, D> From<D> for Dagfs<S>
where
    S: Store,
    Dagio<S>: From<D>,
{
    fn from(d: D) -> Self {
        Dagfs(Dagio::from(d))
    }
}

impl<S> Dagfs<S> where S: Store {}
