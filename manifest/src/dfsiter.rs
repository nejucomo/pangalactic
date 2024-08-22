use pangalactic_dir::Name;
use pangalactic_link::Link;
use pangalactic_nested_dir::DfsIter as Inner;

#[derive(Debug, derive_more::From)]
pub struct DfsIter<C>(Inner<C, C>);

impl<C> Iterator for DfsIter<C> {
    type Item = (Vec<Name>, Link<C>);

    fn next(&mut self) -> Option<Self::Item> {
        use pangalactic_linkkind::LinkKind::{Dir, File};

        self.0.next().map(|(path, n)| {
            (
                path,
                n.either(|cid| Link::new(File, cid), |cid| Link::new(Dir, cid)),
            )
        })
    }
}
