use pangalactic_node::Node;
use pangalactic_runopt::{Application, RunApp};

use crate::options::{Command, Options, PubCapCommand, PubCapOptions};
use crate::pcna::{PubCapNodeApp, PubCapNodeExt as _};

impl Application for Options {
    async fn run(self) -> anyhow::Result<()> {
        let node = Node::from(self.nodeopts);
        self.command.run_app(node).await?;
        Ok(())
    }
}

impl RunApp<Node> for Command {
    async fn run_app(self, node: Node) -> anyhow::Result<()> {
        use Command::*;

        match self {
            PubCap(opts) => opts.run_app(node).await,
        }
    }
}

impl RunApp<Node> for PubCapOptions {
    async fn run_app(self, node: Node) -> anyhow::Result<()> {
        self.command
            .run_app(node.with_pubcap_dir(self.pubcap_dir))
            .await
    }
}

impl RunApp<PubCapNodeApp> for PubCapCommand {
    async fn run_app(self, app: PubCapNodeApp) -> anyhow::Result<()> {
        use PubCapCommand::*;

        match self {
            Generate => app.generate().await,
            GetSubscribeCap => app.get_subscribe_cap().await,
        }
    }
}
