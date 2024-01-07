use crate::io::*;

#[derive(new)]
pub struct CmdExecutor<'a> {
    pub command: String,
    pub executor: &'a (dyn PipedCmdExecute + Send + Sync),
}

#[async_trait::async_trait]
impl<'a> super::Component for CmdExecutor<'a> {
    type Output = anyhow::Result<Exit>;

    async fn handle(&self) -> Self::Output {
        let output = self.executor.piped_exec(self.command.as_str()).await?;
        Ok(output)
    }
}
