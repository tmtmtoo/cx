use crate::exec::*;
use crate::prelude::*;

#[derive(new)]
pub struct CmdExecutor {
    pub command: String,
    pub executor: Arc<dyn PipedCmdExecutor + Send + Sync>,
}

#[async_trait]
impl super::Component for CmdExecutor {
    type Output = Result<Exit>;

    async fn handle(&self) -> Self::Output {
        let output = self.executor.piped_exec(self.command.as_str()).await?;
        Ok(output)
    }
}

#[derive(new)]
pub struct PrintableCmdNotFound<C> {
    pub command: String,
    pub inner: C,
}

#[async_trait]
impl<T: 'static, C: super::Component<Output = Result<T>> + Send + Sync> super::Component
    for PrintableCmdNotFound<C>
{
    type Output = Result<T>;

    async fn handle(&self) -> Self::Output {
        let result = self.inner.handle().await;

        match &result {
            Err(_) => {
                if self.command.is_empty() {
                    eprintln!("cx: no command entered")
                } else {
                    eprintln!(
                        "cx: command not found '{}'",
                        self.command
                            .split(" ")
                            .collect::<Vec<_>>()
                            .get(0)
                            .unwrap_or(&"")
                    )
                }
            }
            _ => (),
        };

        result
    }
}

pub struct WaitSec {
    pub sec: f64,
}

#[async_trait]
impl super::Component for WaitSec {
    type Output = ();

    async fn handle(&self) -> Self::Output {
        tokio::time::sleep(tokio::time::Duration::from_secs_f64(self.sec)).await
    }
}
