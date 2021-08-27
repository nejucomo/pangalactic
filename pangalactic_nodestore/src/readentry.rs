use pangalactic_node::Dir;
use pangalactic_store::Store;

pub enum ReadEntry<S>
where
    S: Store,
{
    Dir(Dir<<S as Store>::Key>),
    FileStream(<S as Store>::Reader),
}
