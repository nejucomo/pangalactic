use async_trait::async_trait;

use crate::{
    cmd::StoreCommander,
    options::{Command, Options, StoreCommand, StoreGetOptions, StorePutOptions, StoreXferOptions},
};

#[cfg_attr(not(doc), async_trait)]
pub trait Runnable {
    async fn run(self) -> anyhow::Result<()>;
}

#[cfg_attr(not(doc), async_trait)]
impl Runnable for Options {
    async fn run(self) -> anyhow::Result<()> {
        self.command.unwrap().run().await
    }
}

#[cfg_attr(not(doc), async_trait)]
impl Runnable for Command {
    async fn run(self) -> anyhow::Result<()> {
        use Command::*;

        match self {
            Store(x) => x.run().await,
        }
    }
}

#[cfg_attr(not(doc), async_trait)]
impl Runnable for StoreCommand {
    async fn run(self) -> anyhow::Result<()> {
        use StoreCommand::*;

        match self {
            Put(x) => x.run().await,
            Get(x) => x.run().await,
            Xfer(x) => x.run().await,
        }
    }
}

#[cfg_attr(not(doc), async_trait)]
impl Runnable for StorePutOptions {
    async fn run(self) -> anyhow::Result<()> {
        let mut sc = StoreCommander::default();
        let link = sc.put().await?;
        println!("{link}");
        Ok(())
    }
}

#[cfg_attr(not(doc), async_trait)]
impl Runnable for StoreGetOptions {
    async fn run(self) -> anyhow::Result<()> {
        let mut sc = StoreCommander::default();
        sc.get(&self.link).await
    }
}

#[cfg_attr(not(doc), async_trait)]
impl Runnable for StoreXferOptions {
    async fn run(self) -> anyhow::Result<()> {
        let mut sc = StoreCommander::default();
        if let Some(link) = sc.xfer(&self.source, &self.dest).await? {
            println!("{link}");
        }
        Ok(())
    }
}
