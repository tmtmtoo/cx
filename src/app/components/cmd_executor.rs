use crate::exec::*;

#[derive(new)]
pub struct CmdExecutor {
    pub command: String,
    pub executor: std::sync::Arc<dyn PipedCmdExecutor + Send + Sync>,
}

#[async_trait::async_trait]
impl crate::app::Component for CmdExecutor {
    type Output = anyhow::Result<Exit>;

    async fn handle(&self) -> Self::Output {
        let output = self.executor.piped_exec(self.command.as_str()).await?;
        Ok(output)
    }
}
