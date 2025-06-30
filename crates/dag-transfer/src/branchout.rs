use anyhow::Result;
use extend::ext;
use pangalactic_name::Name;

#[ext(name = BranchIterOutput)]
pub impl<T> Result<Option<(Name, T)>> {
    fn map_branch_item<F, U>(self, f: F) -> Result<Option<(Name, U)>>
    where
        F: FnOnce(T) -> U,
    {
        if let Some((name, item)) = self? {
            Ok(Some((name, f(item))))
        } else {
            Ok(None)
        }
    }
}
