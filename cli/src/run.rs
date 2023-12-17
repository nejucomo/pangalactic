use crate::options::Options;

pub async fn run() -> anyhow::Result<()> {
    let opts = Options::parse();
    dbg!(opts);
    todo!()
}
