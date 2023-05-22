use pangalactic_dir::Directory;

#[derive(Debug)]
pub struct Plan<L> {
    pub exec: L,
    pub input: L,
}

impl<L> From<Plan<L>> for Directory<L> {
    fn from(plan: Plan<L>) -> Self {
        let mut d = Directory::default();
        d.insert("exec".to_string(), plan.exec).unwrap();
        d.insert("input".to_string(), plan.input).unwrap();
        d
    }
}

impl<L> TryFrom<Directory<L>> for Plan<L> {
    type Error = anyhow::Error;

    fn try_from(mut dir: Directory<L>) -> anyhow::Result<Self> {
        let exec = dir.remove_required("exec")?;
        let input = dir.remove_required("input")?;
        dir.require_empty()?;
        Ok(Plan { exec, input })
    }
}
