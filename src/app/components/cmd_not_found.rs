use crate::app::*;

#[derive(new)]
pub struct PrintableCmdNotFound<C> {
    pub command: String,
    pub inner: C,
}

#[async_trait::async_trait]
impl<T: 'static, C: Component<Output = anyhow::Result<T>> + Send + Sync>
    Component for PrintableCmdNotFound<C>
{
    type Output = anyhow::Result<T>;

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
