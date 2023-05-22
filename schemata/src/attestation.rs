use pangalactic_dir::Directory;

#[derive(Debug)]
pub struct Attestation<L> {
    pub plan: L,
    pub output: L,
}

impl<L> From<Attestation<L>> for Directory<L> {
    fn from(att: Attestation<L>) -> Self {
        let mut d = Directory::default();
        d.insert("plan".to_string(), att.plan).unwrap();
        d.insert("output".to_string(), att.output).unwrap();
        d
    }
}

impl<L> TryFrom<Directory<L>> for Attestation<L> {
    type Error = anyhow::Error;

    fn try_from(mut dir: Directory<L>) -> anyhow::Result<Self> {
        let plan = dir.remove_required("plan")?;
        let output = dir.remove_required("output")?;
        dir.require_empty()?;
        Ok(Attestation { plan, output })
    }
}
