use pangalactic_dagnode::DagNode;
use pangalactic_store::Store;

pub struct VirtualMachine<S>
where
    S: Store,
{
    store: S,
    writers: Vec<Option<DagNode<<S as Store>::Key>>>,
    readers: Vec<Option<DagNode<<S as Store>::Key>>>,
    links: Vec<<S as Store>::Key>,
}
